<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import type { FileAttachment } from '../stores/document';

  export let attachments: FileAttachment[] = [];
  export let group: string = '';

  const dispatch = createEventDispatcher<{
    update: FileAttachment[];
  }>();

  $: filtered = group ? attachments.filter(a => a.group === group) : attachments;

  const MAX_FILE_SIZE = 10 * 1024 * 1024; // 10MB
  const ALLOWED_EXTENSIONS = new Set(['pdf', 'png', 'jpg', 'jpeg', 'gif', 'webp', 'doc', 'docx', 'txt', 'xls', 'xlsx', 'csv']);
  const ALLOWED_EXTENSIONS_LIST = [...ALLOWED_EXTENSIONS];

  let error = '';
  let fileInput: HTMLInputElement;

  const isTauri = typeof window !== 'undefined' && !!(window as any).__TAURI_INTERNALS__;

  function generateId(): string {
    return Math.random().toString(36).substring(2, 9);
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1048576) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / 1048576).toFixed(1) + ' MB';
  }

  function getMimeType(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() || '';
    const mimeMap: Record<string, string> = {
      pdf: 'application/pdf',
      png: 'image/png',
      jpg: 'image/jpeg',
      jpeg: 'image/jpeg',
      gif: 'image/gif',
      webp: 'image/webp',
      doc: 'application/msword',
      docx: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
      txt: 'text/plain',
      xls: 'application/vnd.ms-excel',
      xlsx: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
      csv: 'text/csv',
    };
    return mimeMap[ext] || 'application/octet-stream';
  }

  function uint8ArrayToBase64(bytes: Uint8Array): string {
    let binary = '';
    for (let i = 0; i < bytes.length; i++) {
      binary += String.fromCharCode(bytes[i]);
    }
    return btoa(binary);
  }

  function getExtension(filename: string): string {
    // Strip query params and fragments, then get extension
    const clean = filename.split(/[?#]/)[0];
    return (clean.split('.').pop() || '').toLowerCase();
  }

  function isContentUri(path: string): boolean {
    return path.startsWith('content://') || path.includes('%3A') || path.includes('document:');
  }

  // Detect file type from first bytes (magic number)
  function detectMimeFromBytes(bytes: Uint8Array): { mime: string; ext: string } | null {
    if (bytes.length < 4) return null;
    const h = (bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3];
    const h2 = (bytes[0] << 8) | bytes[1];

    // PDF: %PDF
    if (h === 0x25504446) return { mime: 'application/pdf', ext: 'pdf' };
    // PNG: 0x89504E47
    if (h === 0x89504E47) return { mime: 'image/png', ext: 'png' };
    // JPEG: 0xFFD8FF
    if ((h >>> 8) === 0xFFD8FF) return { mime: 'image/jpeg', ext: 'jpg' };
    // GIF: GIF8
    if (h === 0x47494638) return { mime: 'image/gif', ext: 'gif' };
    // WEBP: RIFF....WEBP
    if (h === 0x52494646 && bytes.length >= 12 && bytes[8] === 0x57 && bytes[9] === 0x45 && bytes[10] === 0x42 && bytes[11] === 0x50) return { mime: 'image/webp', ext: 'webp' };
    // ZIP-based (docx, xlsx, etc): PK\x03\x04
    if (h === 0x504B0304) {
      // Check for Office Open XML markers in the zip
      const text = new TextDecoder('ascii', { fatal: false }).decode(bytes.subarray(0, Math.min(bytes.length, 2000)));
      if (text.includes('word/')) return { mime: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document', ext: 'docx' };
      if (text.includes('xl/')) return { mime: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet', ext: 'xlsx' };
      // Generic zip - could be docx/xlsx, allow it
      return { mime: 'application/zip', ext: 'xlsx' };
    }
    // OLE2 (doc, xls): 0xD0CF11E0
    if (h === 0xD0CF11E0) return { mime: 'application/msword', ext: 'doc' };

    return null;
  }

  function validateExtension(ext: string): boolean {
    if (!ext || !ALLOWED_EXTENSIONS.has(ext)) {
      error = `Unsupported file type ".${ext || '(none)'}". Supported: ${ALLOWED_EXTENSIONS_LIST.join(', ')}.`;
      return false;
    }
    return true;
  }

  async function handleAttach() {
    error = '';

    if (isTauri) {
      try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const { readFile } = await import('@tauri-apps/plugin-fs');

        const selected = await open({
          multiple: false,
          filters: [
            { name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'] },
            { name: 'Documents', extensions: ['pdf', 'doc', 'docx', 'txt'] },
            { name: 'Spreadsheets', extensions: ['xls', 'xlsx', 'csv'] },
            { name: 'All Supported', extensions: ALLOWED_EXTENSIONS_LIST },
          ],
        });

        if (!selected) return;

        const path = typeof selected === 'string' ? selected : (selected as any).path ?? selected;
        const pathStr = String(path);

        const bytes = await readFile(pathStr);
        if (bytes.length > MAX_FILE_SIZE) {
          error = 'File is too large. Maximum file size is 10 MB.';
          return;
        }

        const rawBytes = bytes instanceof Uint8Array ? bytes : new Uint8Array(bytes);
        let filename: string;
        let mimeType: string;

        if (isContentUri(pathStr)) {
          // Android content URI: detect type from file magic bytes
          const detected = detectMimeFromBytes(rawBytes);
          if (!detected) {
            // Try plain text heuristic: if all bytes are valid UTF-8 printable chars
            const isText = rawBytes.length > 0 && rawBytes.every(b => b === 0x0A || b === 0x0D || b === 0x09 || (b >= 0x20 && b <= 0x7E));
            if (isText) {
              filename = 'file.txt';
              mimeType = 'text/plain';
            } else {
              error = `Could not determine file type. Supported: ${ALLOWED_EXTENSIONS_LIST.join(', ')}.`;
              return;
            }
          } else {
            if (!validateExtension(detected.ext)) return;
            filename = 'file.' + detected.ext;
            mimeType = detected.mime;
          }
        } else {
          // Normal file path: extract filename and validate extension
          filename = pathStr.split(/[/\\]/).pop() || 'file';
          const ext = getExtension(filename);
          if (!validateExtension(ext)) return;
          mimeType = getMimeType(filename);
        }

        const base64 = uint8ArrayToBase64(rawBytes);

        const attachment: FileAttachment = {
          id: generateId(),
          name: filename,
          mime_type: mimeType,
          size: rawBytes.length,
          data: base64,
          group,
        };

        dispatch('update', [...attachments, attachment]);
      } catch (e) {
        console.error('Failed to attach file:', e);
        error = 'Failed to read file.';
      }
    } else {
      // Web fallback: use hidden file input
      fileInput?.click();
    }
  }

  function handleFileInputChange(e: Event) {
    const input = e.target as HTMLInputElement;
    const file = input.files?.[0];
    if (!file) return;

    error = '';

    if (!validateExtension(getExtension(file.name))) {
      input.value = '';
      return;
    }

    if (file.size > MAX_FILE_SIZE) {
      error = 'File is too large. Maximum file size is 10 MB.';
      input.value = '';
      return;
    }

    const reader = new FileReader();
    reader.onload = () => {
      const dataUrl = reader.result as string;
      // dataUrl is "data:<mime>;base64,<data>"
      const base64 = dataUrl.split(',')[1] || '';
      const attachment: FileAttachment = {
        id: generateId(),
        name: file.name,
        mime_type: file.type || getMimeType(file.name),
        size: file.size,
        data: base64,
        group,
      };
      dispatch('update', [...attachments, attachment]);
    };
    reader.onerror = () => {
      error = 'Failed to read file.';
    };
    reader.readAsDataURL(file);
    input.value = '';
  }

  function removeAttachment(id: string) {
    dispatch('update', attachments.filter(a => a.id !== id));
  }

  function downloadAttachment(attachment: FileAttachment) {
    const link = document.createElement('a');
    link.href = `data:${attachment.mime_type};base64,${attachment.data}`;
    link.download = attachment.name;
    link.click();
  }

  function getFileIcon(mimeType: string): string {
    if (mimeType === 'application/pdf') return 'PDF';
    if (mimeType.startsWith('image/')) return 'IMG';
    if (mimeType.includes('word') || mimeType.includes('document')) return 'DOC';
    if (mimeType.includes('sheet') || mimeType.includes('excel') || mimeType === 'text/csv') return 'XLS';
    if (mimeType === 'text/plain') return 'TXT';
    return 'FILE';
  }
</script>

<div class="file-attachments">
  <button class="attach-btn" on:click={handleAttach}>+ Attach File</button>

  {#if error}
    <div class="attach-error">{error}</div>
  {/if}

  {#if filtered.length > 0}
    <div class="attachment-list">
      {#each filtered as attachment (attachment.id)}
        <div class="attachment-item">
          <div class="attachment-icon">{getFileIcon(attachment.mime_type)}</div>
          <div class="attachment-info">
            <button class="attachment-name" on:click={() => downloadAttachment(attachment)} title="Download">{attachment.name}</button>
            <span class="attachment-size">{formatSize(attachment.size)}</span>
          </div>
          <button class="attachment-remove" on:click={() => removeAttachment(attachment.id)} title="Remove">&times;</button>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Hidden file input for web fallback -->
  <input
    type="file"
    bind:this={fileInput}
    on:change={handleFileInputChange}
    accept={ALLOWED_EXTENSIONS_LIST.map(e => '.' + e).join(',')}
    style="display: none;"
  />
</div>

<style>
  .file-attachments {
    margin-top: 12px;
  }

  .attach-btn {
    padding: 4px 10px;
    font-size: 0.8rem;
    font-weight: 500;
    border: 1px dashed var(--accent-primary);
    border-radius: 6px;
    background: transparent;
    color: var(--accent-primary);
    cursor: pointer;
    transition: background 0.15s;
  }

  .attach-btn:hover {
    background: var(--accent-light);
  }

  .attach-error {
    color: var(--error-color);
    font-size: 0.85rem;
    margin-bottom: 8px;
  }

  .attachment-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .attachment-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 10px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-color);
    border-radius: 6px;
  }


  .attachment-icon {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-tertiary);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    font-size: 0.7rem;
    font-weight: 700;
    color: var(--text-secondary);
    flex-shrink: 0;
  }

  .attachment-info {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
  }

  .attachment-name {
    font-size: 0.9rem;
    color: var(--accent-primary);
    cursor: pointer;
    background: none;
    border: none;
    padding: 0;
    text-align: left;
    text-decoration: underline;
    text-decoration-color: transparent;
    transition: text-decoration-color 0.15s;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .attachment-name:hover {
    text-decoration-color: var(--accent-primary);
  }

  .attachment-size {
    font-size: 0.75rem;
    color: var(--text-secondary);
  }

  .attachment-remove {
    background: none;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    width: 28px;
    height: 28px;
    cursor: pointer;
    font-size: 1.1rem;
    color: var(--text-secondary);
    flex-shrink: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.15s;
  }

  .attachment-remove:hover {
    background: rgba(155, 44, 44, 0.1);
    border-color: var(--error-color);
    color: var(--error-color);
  }
</style>

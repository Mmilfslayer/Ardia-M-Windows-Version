import { invoke } from '@tauri-apps/api/core';

export interface Bookmark {
  id: string;
  parentId?: string;
  index: number;
  title: string;
  url?: string;
  dateAdded: number;
  dateModified?: number;
  isFolder: boolean;
  children?: Bookmark[];
}

export interface BookmarkTree extends Bookmark {}

export async function getBookmarkTree(profileId: string): Promise<BookmarkTree[]> {
  return invoke('bookmark_get_tree', { profile_id: profileId });
}

export async function addBookmark(
  parentId: string,
  index: number,
  title: string,
  url: string | undefined,
  profileId: string
): Promise<string> {
  return invoke('bookmark_add', { request: { parent_id: parentId, index, title, url, profile_id: profileId } });
}

export async function addFolder(
  parentId: string,
  index: number,
  title: string,
  profileId: string
): Promise<string> {
  return invoke('bookmark_add', { request: { parent_id: parentId, index, title, url: undefined, profile_id: profileId } });
}

export async function updateBookmark(id: string, title: string | undefined, url: string | undefined): Promise<void> {
  return invoke('bookmark_update', { id, title, url });
}

export async function moveBookmark(id: string, newParentId: string, newIndex: number): Promise<void> {
  return invoke('bookmark_move', { id, new_parent_id: newParentId, new_index: newIndex });
}

export async function deleteBookmark(id: string): Promise<void> {
  return invoke('bookmark_remove', { id });
}

export async function searchBookmarks(profileId: string, query: string): Promise<Bookmark[]> {
  return invoke('bookmark_search', { request: { profile_id: profileId, query } });
}

export async function exportBookmarks(profileId: string): Promise<string> {
  return invoke('bookmark_export', { profile_id: profileId });
}

export async function importBookmarks(profileId: string, json: string): Promise<void> {
  return invoke('bookmark_import', { profile_id: profileId, json });
}

export async function getBookmarkBarFolderId(profileId: string): Promise<string> {
  const tree = await getBookmarkTree(profileId);
  const bar = tree.find(t => t.title === 'Bookmarks Bar');
  return bar?.id || '';
}

export async function getOtherBookmarksFolderId(profileId: string): Promise<string> {
  const tree = await getBookmarkTree(profileId);
  const other = tree.find(t => t.title === 'Other Bookmarks');
  return other?.id || '';
}

export async function getMobileBookmarksFolderId(profileId: string): Promise<string> {
  const tree = await getBookmarkTree(profileId);
  const mobile = tree.find(t => t.title === 'Mobile Bookmarks');
  return mobile?.id || '';
}
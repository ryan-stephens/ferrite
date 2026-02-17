interface ToolbarProps {
  search: string;
  sort: string;
  onSearchChange: (v: string) => void;
  onSortChange: (v: string) => void;
  onAddLibrary: () => void;
  onRefresh: () => void;
}

export default function Toolbar(props: ToolbarProps) {
  return (
    <div class="flex gap-3 mb-6 items-center flex-wrap">
      <button
        class="bg-ferrite-500 hover:bg-ferrite-600 text-white font-medium px-4 py-2 rounded-md transition-colors"
        onClick={props.onAddLibrary}
      >
        + Add Library
      </button>
      <button
        class="bg-surface-300 hover:bg-surface-400 text-gray-300 font-medium px-4 py-2 rounded-md transition-colors"
        onClick={props.onRefresh}
      >
        Refresh
      </button>
      <div class="flex-1 min-w-[200px] max-w-[400px] relative">
        <span class="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 text-sm pointer-events-none">🔍</span>
        <input
          id="search-input"
          type="text"
          class="w-full bg-surface-200 border border-surface-300 rounded-md pl-9 pr-3 py-2 text-gray-200 text-sm focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
          placeholder="Search... (/ to focus)"
          value={props.search}
          onInput={e => props.onSearchChange(e.currentTarget.value)}
        />
      </div>
      <select
        class="bg-surface-200 border border-surface-300 rounded-md px-3 py-2 text-gray-200 text-sm min-w-[140px] focus:border-ferrite-500 focus:ring-1 focus:ring-ferrite-500"
        value={props.sort}
        onChange={e => props.onSortChange(e.currentTarget.value)}
      >
        <option value="title-asc">Title A-Z</option>
        <option value="title-desc">Title Z-A</option>
        <option value="year-desc">Year (Newest)</option>
        <option value="year-asc">Year (Oldest)</option>
        <option value="rating-desc">Rating (Best)</option>
        <option value="added-desc">Recently Added</option>
        <option value="played-desc">Recently Played</option>
      </select>
    </div>
  );
}

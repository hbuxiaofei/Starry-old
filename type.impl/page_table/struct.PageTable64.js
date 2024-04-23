(function() {var type_impls = {
"page_table":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PageTable64%3CM,+PTE,+IF%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#39-337\">source</a><a href=\"#impl-PageTable64%3CM,+PTE,+IF%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;M: <a class=\"trait\" href=\"page_table/trait.PagingMetaData.html\" title=\"trait page_table::PagingMetaData\">PagingMetaData</a>, PTE: <a class=\"trait\" href=\"page_table_entry/trait.GenericPTE.html\" title=\"trait page_table_entry::GenericPTE\">GenericPTE</a>, IF: <a class=\"trait\" href=\"page_table/trait.PagingIf.html\" title=\"trait page_table::PagingIf\">PagingIf</a>&gt; <a class=\"struct\" href=\"page_table/struct.PageTable64.html\" title=\"struct page_table::PageTable64\">PageTable64</a>&lt;M, PTE, IF&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_new\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#43-50\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.try_new\" class=\"fn\">try_new</a>() -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a>&lt;Self&gt;</h4></section></summary><div class=\"docblock\"><p>Creates a new page table instance or returns the error.</p>\n<p>It will allocate a new page for the root page table.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.root_paddr\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#53-55\">source</a><h4 class=\"code-header\">pub const fn <a href=\"page_table/struct.PageTable64.html#tymethod.root_paddr\" class=\"fn\">root_paddr</a>(&amp;self) -&gt; <a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a></h4></section></summary><div class=\"docblock\"><p>Returns the physical address of the root page table.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#66-79\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.map\" class=\"fn\">map</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    target: <a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>,\n    page_size: <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Maps a virtual page to a physical frame with the given <code>page_size</code>\nand mapping <code>flags</code>.</p>\n<p>The virtual page starts with <code>vaddr</code>, amd the physical frame starts with\n<code>target</code>. If the addresses is not aligned to the page size, they will be\naligned down automatically.</p>\n<p>Returns <a href=\"page_table/enum.PagingError.html#variant.AlreadyMapped\" title=\"variant page_table::PagingError::AlreadyMapped\"><code>Err(PagingError::AlreadyMapped)</code></a>\nif the mapping is already present.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_overwrite\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#82-97\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.map_overwrite\" class=\"fn\">map_overwrite</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    target: <a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>,\n    page_size: <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Same as <code>PageTable64::map()</code>. This function will error if entry doesn’t exist. Should be\nused to edit PTE in page fault handler.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unmap\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#103-111\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.unmap\" class=\"fn\">unmap</a>(&amp;mut self, vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a>&lt;(<a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>, <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>)&gt;</h4></section></summary><div class=\"docblock\"><p>Unmaps the mapping starts with <code>vaddr</code>.</p>\n<p>Returns <a href=\"page_table/enum.PagingError.html#variant.NotMapped\" title=\"variant page_table::PagingError::NotMapped\"><code>Err(PagingError::NotMapped)</code></a> if the\nmapping is not present.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_fault\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#114-126\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.map_fault\" class=\"fn\">map_fault</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    page_size: <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Maps a fault page starts with <code>vaddr</code>.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.query\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#135-142\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.query\" class=\"fn\">query</a>(\n    &amp;self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a>&lt;(<a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>, <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>, <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>)&gt;</h4></section></summary><div class=\"docblock\"><p>Query the result of the mapping starts with <code>vaddr</code>.</p>\n<p>Returns the physical address of the target frame, mapping flags, and\nthe page size.</p>\n<p>Returns <a href=\"page_table/enum.PagingError.html#variant.NotMapped\" title=\"variant page_table::PagingError::NotMapped\"><code>Err(PagingError::NotMapped)</code></a> if the\nmapping is not present.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.update\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#151-168\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.update\" class=\"fn\">update</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    paddr: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>&gt;,\n    flags: <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>&gt;\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a>&lt;<a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>&gt;</h4></section></summary><div class=\"docblock\"><p>Updates the target or flags of the mapping starts with <code>vaddr</code>. If the\ncorresponding argument is <code>None</code>, it will not be updated.</p>\n<p>Returns the page size of the mapping.</p>\n<p>Returns <a href=\"page_table/enum.PagingError.html#variant.NotMapped\" title=\"variant page_table::PagingError::NotMapped\"><code>Err(PagingError::NotMapped)</code></a> if the\nmapping is not present.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_region\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#181-236\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.map_region\" class=\"fn\">map_region</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    paddr: <a class=\"struct\" href=\"memory_addr/struct.PhysAddr.html\" title=\"struct memory_addr::PhysAddr\">PhysAddr</a>,\n    size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>,\n    allow_huge: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.bool.html\">bool</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Map a contiguous virtual memory region to a contiguous physical memory\nregion with the given mapping <code>flags</code>.</p>\n<p>The virtual and physical memory regions start with <code>vaddr</code> and <code>paddr</code>\nrespectively. The region size is <code>size</code>. The addresses and <code>size</code> must\nbe aligned to 4K, otherwise it will return <a href=\"page_table/enum.PagingError.html#variant.NotAligned\" title=\"variant page_table::PagingError::NotAligned\"><code>Err(PagingError::NotAligned)</code></a>.</p>\n<p>When <code>allow_huge</code> is true, it will try to map the region with huge pages\nif possible. Otherwise, it will map the region with 4K pages.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.map_fault_region\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#239-273\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.map_fault_region\" class=\"fn\">map_fault_region</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>TODO: huge page</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.unmap_region\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#279-298\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.unmap_region\" class=\"fn\">unmap_region</a>(&amp;mut self, vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>, size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Unmap a contiguous virtual memory region.</p>\n<p>The region must be mapped before using <a href=\"page_table/struct.PageTable64.html#method.map_region\" title=\"method page_table::PageTable64::map_region\"><code>PageTable64::map_region</code></a>, or\nunexpected behaviors may occur.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.update_region\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#302-314\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.update_region\" class=\"fn\">update_region</a>(\n    &amp;mut self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>,\n    size: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>,\n    flags: <a class=\"struct\" href=\"page_table_entry/struct.MappingFlags.html\" title=\"struct page_table_entry::MappingFlags\">MappingFlags</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a></h4></section></summary><div class=\"docblock\"><p>Update the mapping flags of a contiguous virtual memory region.\nThe region must be mapped before using <a href=\"page_table/struct.PageTable64.html#method.map_region\" title=\"method page_table::PageTable64::map_region\"><code>PageTable64::map_region</code></a>, or it will return an error.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.walk\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#325-336\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.walk\" class=\"fn\">walk</a>&lt;F&gt;(&amp;self, limit: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, func: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;F</a>) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a><span class=\"where fmt-newline\">where\n    F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/function/trait.Fn.html\" title=\"trait core::ops::function::Fn\">Fn</a>(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;PTE</a>),</span></h4></section></summary><div class=\"docblock\"><p>Walk the page table recursively.</p>\n<p>When reaching the leaf page table, call <code>func</code> on the current page table\nentry. The max number of enumerations in one table is limited by <code>limit</code>.</p>\n<p>The arguments of <code>func</code> are:</p>\n<ul>\n<li>Current level (starts with <code>0</code>): <code>usize</code></li>\n<li>The index of the entry in the current-level table: <code>usize</code></li>\n<li>The virtual address that is mapped to the entry: <a href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\"><code>VirtAddr</code></a></li>\n<li>The reference of the entry: <a href=\"page_table_entry/trait.GenericPTE.html\" title=\"trait page_table_entry::GenericPTE\"><code>&amp;PTE</code></a></li>\n</ul>\n</div></details></div></details>",0,"page_table::arch::x86_64::X64PageTable","page_table::arch::riscv::Sv39PageTable","page_table::arch::riscv::Sv48PageTable","page_table::arch::aarch64::A64PageTable"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PageTable64%3CM,+PTE,+IF%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#340-467\">source</a><a href=\"#impl-PageTable64%3CM,+PTE,+IF%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;M: <a class=\"trait\" href=\"page_table/trait.PagingMetaData.html\" title=\"trait page_table::PagingMetaData\">PagingMetaData</a>, PTE: <a class=\"trait\" href=\"page_table_entry/trait.GenericPTE.html\" title=\"trait page_table_entry::GenericPTE\">GenericPTE</a>, IF: <a class=\"trait\" href=\"page_table/trait.PagingIf.html\" title=\"trait page_table::PagingIf\">PagingIf</a>&gt; <a class=\"struct\" href=\"page_table/struct.PageTable64.html\" title=\"struct page_table::PageTable64\">PageTable64</a>&lt;M, PTE, IF&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.get_entry_mut\" class=\"method\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#383-407\">source</a><h4 class=\"code-header\">pub fn <a href=\"page_table/struct.PageTable64.html#tymethod.get_entry_mut\" class=\"fn\">get_entry_mut</a>(\n    &amp;self,\n    vaddr: <a class=\"struct\" href=\"memory_addr/struct.VirtAddr.html\" title=\"struct memory_addr::VirtAddr\">VirtAddr</a>\n) -&gt; <a class=\"type\" href=\"page_table/type.PagingResult.html\" title=\"type page_table::PagingResult\">PagingResult</a>&lt;(<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.reference.html\">&amp;mut PTE</a>, <a class=\"enum\" href=\"page_table/enum.PageSize.html\" title=\"enum page_table::PageSize\">PageSize</a>)&gt;</h4></section></summary><div class=\"docblock\"><p>To get the mutable reference of the page table entry of the given virtual address.</p>\n</div></details></div></details>",0,"page_table::arch::x86_64::X64PageTable","page_table::arch::riscv::Sv39PageTable","page_table::arch::riscv::Sv48PageTable","page_table::arch::aarch64::A64PageTable"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-PageTable64%3CM,+PTE,+IF%3E\" class=\"impl\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#469-475\">source</a><a href=\"#impl-Drop-for-PageTable64%3CM,+PTE,+IF%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;M: <a class=\"trait\" href=\"page_table/trait.PagingMetaData.html\" title=\"trait page_table::PagingMetaData\">PagingMetaData</a>, PTE: <a class=\"trait\" href=\"page_table_entry/trait.GenericPTE.html\" title=\"trait page_table_entry::GenericPTE\">GenericPTE</a>, IF: <a class=\"trait\" href=\"page_table/trait.PagingIf.html\" title=\"trait page_table::PagingIf\">PagingIf</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for <a class=\"struct\" href=\"page_table/struct.PageTable64.html\" title=\"struct page_table::PageTable64\">PageTable64</a>&lt;M, PTE, IF&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"src/page_table/bits64.rs.html#470-474\">source</a><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","page_table::arch::x86_64::X64PageTable","page_table::arch::riscv::Sv39PageTable","page_table::arch::riscv::Sv48PageTable","page_table::arch::aarch64::A64PageTable"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()
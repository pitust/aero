(function() {var implementors = {};
implementors["aero_boot"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'static mut [<a class=\"struct\" href=\"aero_boot/struct.MemoryRegion.html\" title=\"struct aero_boot::MemoryRegion\">MemoryRegion</a>]&gt; for <a class=\"struct\" href=\"aero_boot/struct.MemoryRegions.html\" title=\"struct aero_boot::MemoryRegions\">MemoryRegions</a>","synthetic":false,"types":["aero_boot::MemoryRegions"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"aero_boot/struct.MemoryRegions.html\" title=\"struct aero_boot::MemoryRegions\">MemoryRegions</a>&gt; for &amp;'static mut [<a class=\"struct\" href=\"aero_boot/struct.MemoryRegion.html\" title=\"struct aero_boot::MemoryRegion\">MemoryRegion</a>]","synthetic":false,"types":["aero_boot::MemoryRegion"]}];
implementors["goblin"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"goblin/container/enum.Container.html\" title=\"enum goblin::container::Container\">Container</a>&gt; for <a class=\"struct\" href=\"goblin/container/struct.Ctx.html\" title=\"struct goblin::container::Ctx\">Ctx</a>","synthetic":false,"types":["goblin::container::Ctx"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"goblin/container/enum.Endian.html\" title=\"enum goblin::container::Endian\">Endian</a>&gt; for <a class=\"struct\" href=\"goblin/container/struct.Ctx.html\" title=\"struct goblin::container::Ctx\">Ctx</a>","synthetic":false,"types":["goblin::container::Ctx"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"goblin/elf/sym/sym32/struct.Sym.html\" title=\"struct goblin::elf::sym::sym32::Sym\">Sym</a>&gt; for <a class=\"struct\" href=\"goblin/elf/sym/struct.Sym.html\" title=\"struct goblin::elf::sym::Sym\">ElfSym</a>","synthetic":false,"types":["goblin::elf::sym::Sym"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"goblin/elf/sym/struct.Sym.html\" title=\"struct goblin::elf::sym::Sym\">Sym</a>&gt; for <a class=\"struct\" href=\"goblin/elf/sym/sym32/struct.Sym.html\" title=\"struct goblin::elf::sym::sym32::Sym\">Sym</a>","synthetic":false,"types":["goblin::elf::sym::sym32::Sym"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"goblin/elf/sym/sym64/struct.Sym.html\" title=\"struct goblin::elf::sym::sym64::Sym\">Sym</a>&gt; for <a class=\"struct\" href=\"goblin/elf/sym/struct.Sym.html\" title=\"struct goblin::elf::sym::Sym\">ElfSym</a>","synthetic":false,"types":["goblin::elf::sym::Sym"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"goblin/elf/sym/struct.Sym.html\" title=\"struct goblin::elf::sym::Sym\">Sym</a>&gt; for <a class=\"struct\" href=\"goblin/elf/sym/sym64/struct.Sym.html\" title=\"struct goblin::elf::sym::sym64::Sym\">Sym</a>","synthetic":false,"types":["goblin::elf::sym::sym64::Sym"]}];
implementors["lock_api"] = [{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a>, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"lock_api/struct.Mutex.html\" title=\"struct lock_api::Mutex\">Mutex</a>&lt;R, T&gt;","synthetic":false,"types":["lock_api::mutex::Mutex"]},{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawMutex.html\" title=\"trait lock_api::RawMutex\">RawMutex</a>, G:&nbsp;<a class=\"trait\" href=\"lock_api/trait.GetThreadId.html\" title=\"trait lock_api::GetThreadId\">GetThreadId</a>, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"lock_api/struct.ReentrantMutex.html\" title=\"struct lock_api::ReentrantMutex\">ReentrantMutex</a>&lt;R, G, T&gt;","synthetic":false,"types":["lock_api::remutex::ReentrantMutex"]},{"text":"impl&lt;R:&nbsp;<a class=\"trait\" href=\"lock_api/trait.RawRwLock.html\" title=\"trait lock_api::RawRwLock\">RawRwLock</a>, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"lock_api/struct.RwLock.html\" title=\"struct lock_api::RwLock\">RwLock</a>&lt;R, T&gt;","synthetic":false,"types":["lock_api::rwlock::RwLock"]}];
implementors["scroll"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;bool&gt; for <a class=\"enum\" href=\"scroll/enum.Endian.html\" title=\"enum scroll::Endian\">Endian</a>","synthetic":false,"types":["scroll::endian::Endian"]}];
implementors["spin"] = [{"text":"impl&lt;T, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"spin/mutex/spin/struct.SpinMutex.html\" title=\"struct spin::mutex::spin::SpinMutex\">SpinMutex</a>&lt;T, R&gt;","synthetic":false,"types":["spin::mutex::spin::SpinMutex"]},{"text":"impl&lt;T, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"spin/mutex/struct.Mutex.html\" title=\"struct spin::mutex::Mutex\">Mutex</a>&lt;T, R&gt;","synthetic":false,"types":["spin::mutex::Mutex"]},{"text":"impl&lt;T, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"spin/once/struct.Once.html\" title=\"struct spin::once::Once\">Once</a>&lt;T, R&gt;","synthetic":false,"types":["spin::once::Once"]},{"text":"impl&lt;T, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"spin/rwlock/struct.RwLock.html\" title=\"struct spin::rwlock::RwLock\">RwLock</a>&lt;T, R&gt;","synthetic":false,"types":["spin::rwlock::RwLock"]}];
implementors["uefi"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;u8&gt; for <a class=\"struct\" href=\"uefi/data_types/chars/struct.Char8.html\" title=\"struct uefi::data_types::chars::Char8\">Char8</a>","synthetic":false,"types":["uefi::data_types::chars::Char8"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"uefi/prelude/struct.Status.html\" title=\"struct uefi::prelude::Status\">Status</a>&gt; for <a class=\"struct\" href=\"uefi/struct.Completion.html\" title=\"struct uefi::Completion\">Completion</a>&lt;()&gt;","synthetic":false,"types":["uefi::result::completion::Completion"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"struct\" href=\"uefi/struct.Completion.html\" title=\"struct uefi::Completion\">Completion</a>&lt;T&gt;","synthetic":false,"types":["uefi::result::completion::Completion"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"uefi/prelude/struct.Status.html\" title=\"struct uefi::prelude::Status\">Status</a>&gt; for <a class=\"type\" href=\"uefi/type.Result.html\" title=\"type uefi::Result\">Result</a>&lt;(), ()&gt;","synthetic":false,"types":["uefi::result::Result"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"ucs2/enum.Error.html\" title=\"enum ucs2::Error\">Error</a>&gt; for <a class=\"struct\" href=\"uefi/prelude/struct.Status.html\" title=\"struct uefi::prelude::Status\">Status</a>","synthetic":false,"types":["uefi::result::status::Status"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;u32&gt; for <a class=\"struct\" href=\"uefi/proto/console/gop/struct.BltPixel.html\" title=\"struct uefi::proto::console::gop::BltPixel\">BltPixel</a>","synthetic":false,"types":["uefi::proto::console::gop::BltPixel"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
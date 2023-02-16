use netcorehost::{nethost, pdcstr};
use netcorehost::hostfxr::AssemblyDelegateLoader;
use netcorehost::pdcstring::PdCStr;

fn main() {
    println!("hello world");
    start_host();
}

pub fn start_host<'a>() -> AssemblyDelegateLoader<&'a PdCStr> {
    fn try_start<'a>() -> Option<AssemblyDelegateLoader<&'a PdCStr>> {
        nethost::load_hostfxr().ok()?
            .initialize_for_runtime_config(pdcstr!("managed.runtimeconfig.json")).ok()?
            .get_delegate_loader_for_assembly(pdcstr!("managed.dll")).ok()
    }

    return match try_start() {
        Some(delegate_loader) => delegate_loader,
        None => panic!("could not start managed dotnet core"),
    };
}

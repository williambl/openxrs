use std::ptr;

use sys::platform::*;

use crate::*;

/// The Headless 'Graphics' API
///
/// See [`XR_MND_headless`] for safety details.
///
/// [`XR_MND_headless`]: https://www.khronos.org/registry/OpenXR/specs/1.0/html/xrspec.html#XR_MND_headless
pub enum Headless {}

impl Graphics for Headless {
    type Requirements = ();
    type SessionCreateInfo = ();
    type Format = ();
    type SwapchainImage = ();

    fn raise_format(x: i64) -> () {
        ()
    }
    fn lower_format(x: ()) -> i64 {
        0
    }

    fn requirements(inst: &Instance, system: SystemId) -> Result<()> {
        Ok(())
    }

    unsafe fn create_session(
        instance: &Instance,
        system: SystemId,
        info: &Self::SessionCreateInfo,
    ) -> Result<sys::Session> {
        let info = sys::SessionCreateInfo {
            ty: sys::SessionCreateInfo::TYPE,
            next: ptr::null(),
            create_flags: Default::default(),
            system_id: system,
        };
        let mut out = sys::Session::NULL;
        cvt((instance.fp().create_session)(
            instance.as_raw(),
            &info,
            &mut out,
        ))?;
        Ok(out)
    }

    fn enumerate_swapchain_images(
        swapchain: &Swapchain<Self>,
    ) -> Result<Vec<Self::SwapchainImage>> {
        Ok(vec![()])
    }
}

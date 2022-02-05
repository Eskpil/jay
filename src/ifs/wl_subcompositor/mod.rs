mod types;

use crate::client::Client;
use crate::globals::{Global, GlobalName};
use crate::ifs::wl_surface::wl_subsurface::WlSubsurface;
use crate::object::{Interface, Object};
use crate::utils::buffd::MsgParser;
use std::rc::Rc;
pub use types::*;

const DESTROY: u32 = 0;
const GET_SUBSURFACE: u32 = 1;

#[allow(dead_code)]
const BAD_SURFACE: u32 = 0;

id!(WlSubcompositorId);

pub struct WlSubcompositorGlobal {
    name: GlobalName,
}

pub struct WlSubcompositor {
    id: WlSubcompositorId,
    client: Rc<Client>,
}

impl WlSubcompositorGlobal {
    pub fn new(name: GlobalName) -> Self {
        Self { name }
    }

    fn bind_(
        self: Rc<Self>,
        id: WlSubcompositorId,
        client: &Rc<Client>,
        _version: u32,
    ) -> Result<(), WlSubcompositorError> {
        let obj = Rc::new(WlSubcompositor {
            id,
            client: client.clone(),
        });
        client.add_client_obj(&obj)?;
        Ok(())
    }
}

impl WlSubcompositor {
    fn destroy(&self, parser: MsgParser<'_, '_>) -> Result<(), DestroyError> {
        let _req: Destroy = self.client.parse(self, parser)?;
        self.client.remove_obj(self)?;
        Ok(())
    }

    fn get_subsurface(&self, parser: MsgParser<'_, '_>) -> Result<(), GetSubsurfaceError> {
        let req: GetSubsurface = self.client.parse(self, parser)?;
        let surface = self.client.lookup(req.surface)?;
        let parent = self.client.lookup(req.parent)?;
        let subsurface = Rc::new(WlSubsurface::new(req.id, &surface, &parent));
        self.client.add_client_obj(&subsurface)?;
        subsurface.install()?;
        Ok(())
    }
}

bind!(WlSubcompositorGlobal);

impl Global for WlSubcompositorGlobal {
    fn name(&self) -> GlobalName {
        self.name
    }

    fn singleton(&self) -> bool {
        true
    }

    fn interface(&self) -> Interface {
        Interface::WlSubcompositor
    }

    fn version(&self) -> u32 {
        1
    }
}

simple_add_global!(WlSubcompositorGlobal);

object_base! {
    WlSubcompositor, WlSubcompositorError;

    DESTROY => destroy,
    GET_SUBSURFACE => get_subsurface,
}

impl Object for WlSubcompositor {
    fn num_requests(&self) -> u32 {
        GET_SUBSURFACE + 1
    }
}

simple_add_obj!(WlSubcompositor);

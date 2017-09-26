#[macro_use]
extern crate bitflags;
extern crate webrender_api;
use webrender_api::{DeviceUintSize, RenderNotifier, PipelineId, Epoch, ExternalImageId, ColorF, BlobImageRenderer, ApiMsg, RenderApiSender, channel};

extern crate gleam;
use gleam::gl;

extern crate rayon;
use rayon::ThreadPool;

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::fmt::Debug;
use std::thread;
use std::fs::File;

pub enum RendererError {
    Io(std::io::Error),
}
impl From<std::io::Error> for RendererError {
    fn from(err: std::io::Error) -> Self {
        RendererError::Io(err)
    }
}
pub struct Renderer {
    gl: Rc<gl::Gl>,
    debug_flags: DebugFlags,
//    api_rx: channel::MsgReceiver<ApiMsg>,
//    payload_rx: channel::PayloadReceiver,
}

impl Renderer {
    pub fn new(gl: Rc<gl::Gl>, options: RendererOptions) -> Result<(Renderer, RenderApiSender), RendererError> {
        let (api_tx, api_rx) = try!{ channel::msg_channel() };
        let (payload_tx, payload_rx) = try!{ channel::payload_channel() };
        let sender = RenderApiSender::new(api_tx, payload_tx);

        let renderer = Renderer {
            gl: gl,
            debug_flags: options.debug_flags,
//            api_rx: api_rx,
//            payload_rx: payload_rx,
        };

        thread::spawn(move || {
            loop {
                match api_rx.recv() {
                    Err(_) => break,
                    Ok(_) => (),
                }
            }
        });
        thread::spawn(move || {
            loop {
                match payload_rx.recv() {
                    Err(_) => break,
                    Ok(_) => (),
                }
            }
        });

        Ok((renderer, sender))
    }
    pub fn update(&self) {}
    pub fn deinit(&self) {}
    pub fn get_debug_flags(&self) -> DebugFlags {
        self.debug_flags
    }

    pub fn set_debug_flags(&mut self, flags: DebugFlags) {
        self.debug_flags = flags;
    }
    pub fn layers_are_bouncing_back(&self) -> bool {
        false
    }
    pub fn render(&mut self, _framebuffer_size: DeviceUintSize) -> Result<(), Vec<RendererError>> {
        Ok(())
    }
    pub fn set_render_notifier(&self, _notifier: Box<RenderNotifier>) {}
    pub fn current_epoch(&self, _pipeline_id: PipelineId) -> Option<Epoch> {
        None
    }
    pub fn set_external_image_handler(&mut self, _handler: Box<ExternalImageHandler>) {
    }
}

// the following come froms the webrender crate

bitflags! {
    #[derive(Default)]
    pub struct DebugFlags: u32 {
        const PROFILER_DBG      = 1 << 0;
        const RENDER_TARGET_DBG = 1 << 1;
        const TEXTURE_CACHE_DBG = 1 << 2;
        const ALPHA_PRIM_DBG    = 1 << 3;
    }
}

pub trait ExternalImageHandler {
    fn lock(&mut self, key: ExternalImageId, channel_index: u8) -> ExternalImage;
    fn unlock(&mut self, key: ExternalImageId, channel_index: u8);
}

pub struct ExternalImage<'a> {
    pub u0: f32,
    pub v0: f32,
    pub u1: f32,
    pub v1: f32,
    pub source: ExternalImageSource<'a>,
}
pub enum ExternalImageSource<'a> {
    RawData(&'a [u8]),
    NativeTexture(u32),
    Invalid,
}
pub struct RendererOptions {
    pub device_pixel_ratio: f32,
    pub resource_override_path: Option<PathBuf>,
    pub enable_aa: bool,
    pub enable_dithering: bool,
    pub max_recorded_profiles: usize,
    pub debug: bool,
    pub enable_scrollbars: bool,
    pub precache_shaders: bool,
    pub renderer_kind: RendererKind,
    pub enable_subpixel_aa: bool,
    pub clear_framebuffer: bool,
    pub clear_color: ColorF,
    pub enable_clear_scissor: bool,
    pub enable_batcher: bool,
    pub max_texture_size: Option<u32>,
    pub workers: Option<Arc<ThreadPool>>,
    pub blob_image_renderer: Option<Box<BlobImageRenderer>>,
    pub recorder: Option<Box<ApiRecordingReceiver>>,
    pub enable_render_on_scroll: bool,
    pub debug_flags: DebugFlags,
}
impl Default for RendererOptions {
    fn default() -> RendererOptions {
        RendererOptions {
            device_pixel_ratio: 1.0,
            resource_override_path: None,
            enable_aa: true,
            enable_dithering: true,
            debug_flags: DebugFlags::empty(),
            max_recorded_profiles: 0,
            debug: false,
            enable_scrollbars: false,
            precache_shaders: false,
            renderer_kind: RendererKind::Native,
            enable_subpixel_aa: false,
            clear_framebuffer: true,
            clear_color: ColorF::new(1.0, 1.0, 1.0, 1.0),
            enable_clear_scissor: true,
            enable_batcher: true,
            max_texture_size: None,
            workers: None,
            blob_image_renderer: None,
            recorder: None,
            enable_render_on_scroll: true,
        }
    }
}
pub enum RendererKind {
    Native,
    OSMesa,
}
pub trait ApiRecordingReceiver: Send + Debug {
    fn write_msg(&mut self, frame: u32, msg: &ApiMsg);
    fn write_payload(&mut self, frame: u32, data: &[u8]);
}
pub struct BinaryRecorder {}
impl BinaryRecorder {
    pub fn new(dest: &PathBuf) -> BinaryRecorder {
        BinaryRecorder {}
    }
}

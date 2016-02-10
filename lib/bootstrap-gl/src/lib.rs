//! Provides a Rust binding to the OpenGL API.
//!
//! This library attempts to provide a complete set of bindings to the OpenGL API while providing
//! a small boost in type-safety over the raw OpenGL API. This library does not abstract the OpenGL
//! API in any way, and in many cases still leaves the task of catching error cases to the
//! programmer. This is meant to be used by other libraries to build higher-level abstractions over
//! raw OpenGL.
//!
//! # Safety Improvements
//!
//! The primary way that `gl-util` improves on the raw OpenGL is by replacing `GLEnum` function
//! parameters with special enum types that only contain variants that are valid options for that
//! function.

#[cfg(target_os="windows")]
#[path="windows\\mod.rs"]
pub mod platform;

#[cfg(target_os = "linux")]
#[path="linux/mod.rs"]
pub mod platform;

pub mod types;

pub use self::types::*;

/// Macro used for generating bindings to OpenGL procs.
///
/// The OpenGL implementation for a computer actually lives in its graphics card. In order to call
/// the various functions that are part of the OpenGL API we must load pointers to those functions.
/// This macro generates the necessary boilerplate for loading and stashing those pointers, as well
/// as handling failure when those pointers fail to load (i.e. panicking).
macro_rules! gl_proc {
    ( $proc_name:ident : fn $fn_name:ident( $( $arg:ident : $arg_ty:ty ),* ) $( -> $result:ty )* ) => {
        pub unsafe fn $fn_name( $( $arg: $arg_ty, )* ) $( -> $result )* {
            $fn_name::$fn_name( $( $arg, )* )
        }

        pub mod $fn_name {
            #[allow(unused_imports)]
            use super::types::*;

            static mut proc_ptr: Option<extern "C" fn( $(
                $arg_ty,
            )* ) $( -> $result )*> = None;

            pub unsafe fn $fn_name( $( $arg: $arg_ty, )* ) $( -> $result )* {
                if let None = proc_ptr {
                    proc_ptr = ::std::mem::transmute(
                        $crate::platform::load_proc(stringify!( $proc_name )));
                }

                match proc_ptr {
                    Some(gl_proc) => gl_proc( $( $arg ),* ),
                    None => panic!("Failed to load gl proc for {}", stringify!( $proc_name )),
                }
            }
        }
    }
}

/// Initializes OpenGl and create the context.
pub fn create_context() -> platform::Context {
    platform::init();
    platform::create_context()
}

/// Destroys and existing OpenGL context.
pub fn destroy_context(context: platform::Context) {
    platform::destroy_context(context);
}

/// Binds a named buffer object.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glBindBuffer)
///
/// Core since version 1.5
///
/// Binds a buffer object to the specified buffer binding point. Calling `bind_buffer` with target
/// set to one of the accepted symbolic constants and buffer​ set to the name of a buffer object
/// binds that buffer object name to the target. If no buffer object with name buffer​ exists one
/// is created with that name. When a buffer object is bound to a target the previous binding for
/// that target is automatically broken.
///
/// Buffer object names are unsigned integers. The value zero is reserved, but there is no default
/// buffer object for each buffer object target. Instead, buffer​ set to zero effectively unbinds
/// any buffer object previously bound, and restores client memory usage for that buffer object
/// target (if supported for that target). Buffer object names and the corresponding buffer object
/// contents are local to the shared object space of the current GL rendering context; two
/// rendering contexts share buffer object names only if they explicitly enable sharing between
/// contexts through the appropriate GL windows interfaces functions.
///
/// `gen_buffers` must be used to generate a set of unused buffer object names.
///
/// A buffer object binding created with `bind_buffer` remains active until a different buffer
/// object name is bound to the same target or until the bound buffer object is deleted with
/// `delete_buffers`.
///
/// Once created, a named buffer object may be re-bound to any target as often as needed. However,
/// the GL implementation may make choices about how to optimize the storage of a buffer object
/// based on its initial binding target.
///
/// # Buffer Targets
///
/// The state of a buffer object immediately after it is first bound is an unmapped zero-sized
/// memory buffer with `GL_READ_WRITE` access and `GL_STATIC_DRAW` usage.
///
/// While a non-zero buffer object name is bound GL operations on the target to which it is bound
/// affect the bound buffer object and queries of the target to which it is bound return state
/// from the bound buffer object. While buffer object name zero is bound, as in the initial state,
/// attempts to modify or query state on the target to which it is bound generates an
/// `GL_INVALID_OPERATION` error.
///
/// When a non-zero buffer object is bound to the `BufferTarget::Array` target the vertex array
/// pointer parameter is interpreted as an offset within the buffer object measured in basic
/// machine units (bytes).
///
/// When a non-zero buffer object is bound to the `BufferTarget::DrawIndirect` target parameters
/// for draws issued through `draw_arrays_indirect` and `draw_elements_indirect` are sourced from
/// the specified offset in that buffer object's data store.
///
/// When a non-zero buffer object is bound to the `BufferTarget::DispatchIndirect` target, the
/// parameters for compute dispatches issued through `dispatch_compute_indirect` are sourced from
/// the specified offset in that buffer object's data store.
///
/// While a non-zero buffer object is bound to the `BufferTarget::ElementArray` target the indices
/// parameter of `draw_elements`, `draw_elements_instanced`, `draw_elements_base_vertex`,
/// `draw_range_elements`, `draw_range_elements_base_vertex`, `multi_draw_elements`, or
/// `multi_draw_elements_base_vertex` is interpreted as an offset within the buffer object
/// measured in basic machine units (bytes).
///
/// While a non-zero buffer object is bound to the `BufferTarget::PixelPack` target the following
/// commands are affected: `get_compressed_tex_image`, `get_tex_image`, and `read_pixels`. The
/// pointer parameter is interpreted as an offset within the buffer object measured in basic
/// machine units (bytes).
///
/// While a non-zero buffer object is bound to the `BufferTarget::PixelUnpack` target the
/// following commands are affected: `compressed_tex_image_1d`, `compressed_tex_image_2d`,
/// `compressed_tex_image_3d`, `compressed_tex_sub_image_1d`, `compressed_tex_sub_image_2d`,
/// `compressed_tex_sub_image_3d`, `tex_image_1d`, `tex_image_2d`, `tex_image_3d`,
/// `tex_sub_image_1d`, `tex_sub_image_2d`, and `tex_sub_image_3d`. The pointer parameter is
/// interpreted as an offset within the buffer object measured in basic machine units (bytes).
///
/// The buffer targets `BufferTarget::CopyRead` and `BufferTarget::CopyWrite` are provided to
/// allow `copy_buffer_sub_data` to be used without disturbing the state of other bindings.
/// However, `copy_buffer_sub_data` may be used with any pair of buffer binding points.
///
/// The `BufferTarget::TransformFeedback` buffer binding point may be passed to `bind_buffer`, but
/// will not directly affect transform feedback state. Instead, the indexed
/// `BufferTarget::TransformFeedback` bindings must be used through a call to `bind_buffer_base` or
/// `bind_buffer_range`. This will affect the generic `BufferTarget::TransformFeedback` binding.
///
/// Likewise, the `BufferTarget::Uniform`, `BufferTarget::AtomicCounter` and
/// `BufferTarget::ShaderStorage` buffer binding points may be used but do not directly affect
/// uniform buffer, atomic counter buffer, or shader storage buffer state, respectively.
/// `bind_buffer_base` or `bind_buffer_range` must be used to bind a buffer to an indexed uniform
/// buffer, atomic counter buffer, or storage buffer binding point.
///
/// The `BufferTarget::Query` binding point is used to specify a buffer object that is to receive
/// the results of query objects through calls to the `get_query_object` family of commands.
///
/// # Version Availability
///
/// - The `Read`, `Uniform`, and `Texture` targets are available only if the GL version is 3.1 or
///   greater.
/// - The `AtomicCounter` target is available only if the GL version is 4.2 or greater.
/// - The `DispatchIndirect` and `ShaderStorage` targets are available only if the GL version is
///   4.3 or greater.
/// - The `Query` target is available only if the GL version is 4.4 or greater.
///
/// # Errors
///
/// - `GL_INVALID_VALUE` is generated if buffer​ is not a name previously returned from a call to
///   `gen_buffers`.
gl_proc!(glBindBuffer:
    fn bind_buffer(target: BufferTarget, buffer: BufferName));

/// Creates and initializes a buffer object's data store.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glBufferData)
///
/// Core since version 1.5
///
/// Creates a new data store for the buffer object currently bound to target​. Any pre-existing
/// data store is deleted. The new data store is created with the specified size​ in bytes and
/// usage​. If data​ is not null, the data store is initialized with data from this pointer. In its
/// initial state the new data store is not mapped, it has a null mapped pointer, and its mapped
/// access is `GL_READ_WRITE`.
///
/// # Buffer Usage
///
/// `usage​` is a hint to the GL implementation as to how a buffer object's data store will be
/// accessed. This enables the GL implementation to make more intelligent decisions that may
/// significantly impact buffer object performance. It does not, however, constrain the actual
/// usage of the data store. usage​ can be broken down into two parts: first, the frequency of
/// access (modification and usage), and second, the nature of that access. The frequency of
/// access may be one of these:
///
/// - **STREAM** - The data store contents will be modified once and used at most a few times.
/// - **STATIC** - The data store contents will be modified once and used many times.
/// - **DYNAMIC** - The data store contents will be modified repeatedly and used many times.
///
/// The nature of access may be one of these:
///
/// - **DRAW** - The data store contents are modified by the application, and used as the source
///   for GL drawing and image specification commands.
/// - **READ** - The data store contents are modified by reading data from the GL, and used to
///   return that data when queried by the application.
/// - **COPY** - The data store contents are modified by reading data from the GL, and used as the
///   source for GL drawing and image specification commands.
///
/// # Notes
///
/// - If `data​` is null a data store of the specified size is still created but its contents remain
///   uninitialized and thus undefined.
/// - Clients must align data elements consistent with the requirements of the client platform, with
///   an additional base-level requirement that an offset within a buffer to a datum comprising N
///   bytes be a multiple of N.
/// - The `AtomicCounter` target is available only if the GL version is 4.2 or greater.
/// - The `DispatchIndirect` and `ShaderStorage` targets are available only if the GL version is
///   4.3 or greater.
/// - The `Query` target is available only if the GL version is 4.4 or greater.
///
/// # Errors
///
/// - `GL_INVALID_VALUE` is generated if `size​` is negative.
/// - `GL_INVALID_OPERATION` is generated if the reserved buffer object name 0 is bound to target​.
/// - `GL_OUT_OF_MEMORY` is generated if the GL is unable to create a data store with the
///   specified size​.
gl_proc!(glBufferData:
    fn buffer_data(target: BufferTarget, size: isize, data: *const (), usage: BufferUsage));

gl_proc!(glClearColor:
    fn clear_color(red: f32, green: f32, blue: f32, alpha: f32));

gl_proc!(glDebugMessageCallback:
    fn debug_message_callback(
        callback: extern "C" fn(DebugSource, DebugType, UInt, DebugSeverity, SizeI, *const u8, *mut ()),
        user_param: *mut ()
    ));

/// Deletes named buffer objects.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glDeleteBuffers)
///
/// Core since version 1.5
///
/// Deletes n​ buffer objects named by the elements of the array buffers​. After a buffer object is
/// deleted, it has no contents, and its name is free for reuse (for example by glGenBuffers​). If
/// a buffer object that is currently bound is deleted, the binding reverts to 0 (the absence of
/// any buffer object).
///
/// glDeleteBuffers silently ignores 0's and names that do not correspond to existing buffer
/// objects.
///
/// # Errors
///
/// `GL_INVALID_VALUE` is generated if `num_buffers` is negative.
gl_proc!(glDeleteBuffers:
    fn delete_buffers(num_buffers: i32, buffers: *const BufferName));

/// Deletes name vertex array objects.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glDeleteVertexArrays)
///
/// Core since version 3.0
///
/// Deletes n​ vertex array objects whose names are stored in the array addressed by arrays​. Once a
/// vertex array object is deleted it has no contents and its name is again unused. If a vertex
/// array object that is currently bound is deleted, the binding for that object reverts to zero
/// and the default vertex array becomes current. Unused names in arrays​ are silently ignored, as
/// is the value zero.
///
/// # Errors
///
/// `GL_INVALID_VALUE` is generated if `num_arrays`​ is negative.
gl_proc!(glDeleteVertexArrays:
    fn delete_vertex_arrays(num_arrays: i32, arrays: *const VertexArrayName));

gl_proc!(glDisable:
    fn disable(capability: ServerCapability));

gl_proc!(glEnable:
    fn enable(capability: ServerCapability));

/// Generates buffer object names.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glGenBuffers)
///
/// Core since version 1.5
///
/// glGenBuffers returns n​ buffer object names in buffers​. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that none of the returned names
/// was in use immediately before the call to glGenBuffers.
///
/// Buffer object names returned by a call to glGenBuffers are not returned by subsequent calls,
/// unless they are first deleted with glDeleteBuffers​.
///
/// No buffer objects are associated with the returned buffer object names until they are first
/// bound by calling glBindBuffer​.
///
/// # Errors
///
/// `GL_INVALID_VALUE` is generated if `num_buffers`​ is negative.
gl_proc!(glGenBuffers:
    fn gen_buffers(num_buffers: i32, buffers: *mut BufferName));

/// Generates vertex array object names.
///
/// [Wiki page](https://www.opengl.org/wiki/GLAPI/glGenVertexArrays)
///
/// Core since version 3.0
///
/// Returns `num_arrays`​ vertex array object names in arrays​. There is no guarantee that the names
/// form a contiguous set of integers; however, it is guaranteed that none of the returned names
/// was in use immediately before the call to `gen_vertex_arrays`.
///
/// Vertex array object names returned by a call to `gen_vertex_arrays` are not returned by
/// subsequent calls, unless they are first deleted with `delete_vertex_arrays`.
///
/// The names returned in arrays​ are marked as used, for the purposes of `gen_vertex_arrays` only,
/// but they acquire state and type only when they are first bound.
///
/// # Errors
///
/// `GL_INVALID_VALUE` is generated if `num_arrays`​ is negative.
gl_proc!(glGenVertexArrays:
    fn gen_vertex_arrays(num_arrays: i32, arrays: *mut VertexArrayName));

/*
gen_proc_loader! {
    glGetError:
        fn get_error() -> ErrorCode,
    glGetIntegerv:
        fn get_integers(name: IntegerName, params: *mut i32),
    glGetString:
        fn get_string(name: StringName) -> *const i8,
    glViewport:
        fn viewport(x: i32, y: i32, width: i32, height: i32),
    glBindVertexArray:
        fn bind_vertex_array(vao: VertexArrayName),
    glBufferData:
        fn buffer_data(target: BufferTarget, size: isize, data: *const (), usage: BufferUsage),
    glClear:
        fn clear(mask: ClearBufferMask),
    glCreateShader:
        fn create_shader(shader_type: ShaderType) -> ShaderObject,
    glShaderSource:
        fn shader_source(
            shader: ShaderObject,
            count: i32,
            strings: *const *const u8,
            length: *const i32),
    glCompileShader:
        fn compile_shader(shader: ShaderObject),
    glGetShaderiv:
        fn get_shader_param(shader: ShaderObject, param_type: ShaderParam, param_out: *mut i32),
    glGetShaderInfoLog:
        fn get_shader_info_log(
            shader: ShaderObject,
            max_length: i32,
            length_out: *mut i32,
            log_out: *mut u8),
    glCreateProgram:
        fn create_program() -> ProgramObject,
    glAttachShader:
        fn attach_shader(program: ProgramObject, shader: ShaderObject),
    glLinkProgram:
        fn link_program(program: ProgramObject),
    glGetProgramiv:
        fn get_program_param(
            program: ProgramObject,
            param_type: ProgramParam,
            param_out: *mut i32),
    glGetProgramInfoLog:
        fn get_program_info_log(
            program: ProgramObject,
            max_length: i32,
            length_out: *mut i32,
            log_out: *mut u8),
    glUseProgram:
        fn use_program(program: ProgramObject),
    glGetAttribLocation:
        fn get_attrib_location(program: ProgramObject, attrib_name: *const i8) -> i32,
    glVertexAttribPointer:
        fn vertex_attrib_pointer(
            attrib: AttributeLocation,
            size: i32,
            gl_type: GLType,
            normalized: bool,
            stride: i32,
            offset: usize),
    glEnableVertexAttribArray:
        fn enable_vertex_attrib_array(attrib: AttributeLocation),
    glGetUniformLocation:
        fn get_uniform_location(program: ProgramObject, uniform_name: *const i8) -> i32,
    glUniform1f:
        fn uniform_1f(uniform: UniformLocation, value: f32),
    glUniformMatrix4fv:
        fn uniform_matrix_4fv(
            uniform: UniformLocation,
            count: i32,
            transpose: bool,
            values: *const f32),
    glUniform4fv:
        fn uniform_4fv(uniform: UniformLocation, count: i32, data: *const f32),
    glDrawElements:
        fn draw_elements(mode: DrawMode, count: i32, index_type: IndexType, offset: usize),
    glDrawArrays:
        fn draw_arrays(mode: DrawMode, first: i32, count: i32),
    glDepthFunc:
        fn depth_func(func: Comparison),
    glBlendFunc:
        fn blend_func(src_factor: SourceFactor, dest_factor: DestFactor),
    glGenTextures:
        fn gen_textures(count: u32, textures: *mut TextureObject),
    glBindTexture:
        fn bind_texture(target: TextureBindTarget, texture: TextureObject),
    glTexImage2D:
        fn texture_image_2d(
            target:          Texture2dTarget,
            level:           i32,
            internal_format: TextureInternalFormat,
            width:           i32,
            height:          i32,
            border:          i32,
            format:          TextureFormat,
            data_type:       TextureDataType,
            data:            *const ()),
    glDeleteTextures:
        fn delete_textures(count: u32, textures: *mut TextureObject),
}
*/

pub extern "C" fn debug_callback(
    source: DebugSource,
    message_type: DebugType,
    _id: UInt,
    severity: DebugSeverity,
    _length: SizeI,
    message: *const u8,
    _user_param: *mut ()
) {
    use std::ffi::CStr;

    println!(
        "Recieved some kind of debug message. source: {:?}, type: {:?}, severity: {:?}, message: {:?}",
        source,
        message_type,
        severity,
        unsafe { CStr::from_ptr(message as *const _) })
}

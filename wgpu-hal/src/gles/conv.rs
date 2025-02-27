impl super::AdapterShared {
    pub(super) fn describe_texture_format(
        &self,
        texture_format: wgt::TextureFormat,
    ) -> super::TextureFormatDesc {
        use wgt::TextureFormat as Tf;

        let (internal, external, data_type) = match texture_format {
            Tf::R8Unorm => (glow::R8, glow::RED, glow::UNSIGNED_BYTE),
            Tf::R8Snorm => (glow::R8, glow::RED, glow::BYTE),
            Tf::R8Uint => (glow::R8UI, glow::RED_INTEGER, glow::UNSIGNED_BYTE),
            Tf::R8Sint => (glow::R8I, glow::RED_INTEGER, glow::BYTE),
            Tf::R16Uint => (glow::R16UI, glow::RED_INTEGER, glow::UNSIGNED_SHORT),
            Tf::R16Sint => (glow::R16I, glow::RED_INTEGER, glow::SHORT),
            Tf::R16Float => (glow::R16F, glow::RED, glow::HALF_FLOAT),
            Tf::Rg8Unorm => (glow::RG8, glow::RG, glow::UNSIGNED_BYTE),
            Tf::Rg8Snorm => (glow::RG8, glow::RG, glow::BYTE),
            Tf::Rg8Uint => (glow::RG8UI, glow::RG_INTEGER, glow::UNSIGNED_BYTE),
            Tf::Rg8Sint => (glow::RG8I, glow::RG_INTEGER, glow::BYTE),
            Tf::R32Uint => (glow::R32UI, glow::RED_INTEGER, glow::UNSIGNED_INT),
            Tf::R32Sint => (glow::R32I, glow::RED_INTEGER, glow::INT),
            Tf::R32Float => (glow::R32F, glow::RED, glow::FLOAT),
            Tf::Rg16Uint => (glow::RG16UI, glow::RG_INTEGER, glow::UNSIGNED_SHORT),
            Tf::Rg16Sint => (glow::RG16I, glow::RG_INTEGER, glow::SHORT),
            Tf::Rg16Float => (glow::RG16F, glow::RG, glow::HALF_FLOAT),
            Tf::Rgba8Unorm => (glow::RGBA8, glow::RGBA, glow::UNSIGNED_BYTE),
            Tf::Rgba8UnormSrgb => (glow::SRGB8_ALPHA8, glow::RGBA, glow::UNSIGNED_BYTE),
            Tf::Bgra8UnormSrgb => (glow::SRGB8_ALPHA8, glow::RGBA, glow::UNSIGNED_BYTE), //TODO?
            Tf::Rgba8Snorm => (glow::RGBA8, glow::RGBA, glow::BYTE),
            Tf::Bgra8Unorm => (glow::RGBA8, glow::BGRA, glow::UNSIGNED_BYTE),
            Tf::Rgba8Uint => (glow::RGBA8UI, glow::RGBA_INTEGER, glow::UNSIGNED_BYTE),
            Tf::Rgba8Sint => (glow::RGBA8I, glow::RGBA_INTEGER, glow::BYTE),
            Tf::Rgb10a2Unorm => (
                glow::RGB10_A2,
                glow::RGBA,
                glow::UNSIGNED_INT_2_10_10_10_REV,
            ),
            Tf::Rg11b10Float => (
                glow::R11F_G11F_B10F,
                glow::RGB,
                glow::UNSIGNED_INT_10F_11F_11F_REV,
            ),
            Tf::Rg32Uint => (glow::RG32UI, glow::RG_INTEGER, glow::UNSIGNED_INT),
            Tf::Rg32Sint => (glow::RG32I, glow::RG_INTEGER, glow::INT),
            Tf::Rg32Float => (glow::RG32F, glow::RG, glow::FLOAT),
            Tf::Rgba16Uint => (glow::RGBA16UI, glow::RGBA_INTEGER, glow::UNSIGNED_SHORT),
            Tf::Rgba16Sint => (glow::RGBA16I, glow::RGBA_INTEGER, glow::SHORT),
            Tf::Rgba16Float => (glow::RGBA16F, glow::RG, glow::HALF_FLOAT),
            Tf::Rgba32Uint => (glow::RGBA32UI, glow::RGBA_INTEGER, glow::UNSIGNED_INT),
            Tf::Rgba32Sint => (glow::RGBA32I, glow::RGBA_INTEGER, glow::INT),
            Tf::Rgba32Float => (glow::RGBA32F, glow::RGBA, glow::FLOAT),
            Tf::Depth32Float => (glow::DEPTH_COMPONENT32F, glow::DEPTH_COMPONENT, glow::FLOAT),
            Tf::Depth24Plus => (
                glow::DEPTH_COMPONENT24,
                glow::DEPTH_COMPONENT,
                glow::UNSIGNED_NORMALIZED,
            ),
            Tf::Depth24PlusStencil8 => (
                glow::DEPTH24_STENCIL8,
                glow::DEPTH_COMPONENT,
                glow::UNSIGNED_INT,
            ),
            Tf::Bc1RgbaUnorm
            | Tf::Bc1RgbaUnormSrgb
            | Tf::Bc2RgbaUnorm
            | Tf::Bc2RgbaUnormSrgb
            | Tf::Bc3RgbaUnorm
            | Tf::Bc3RgbaUnormSrgb
            | Tf::Bc4RUnorm
            | Tf::Bc4RSnorm
            | Tf::Bc5RgUnorm
            | Tf::Bc5RgSnorm
            | Tf::Bc6hRgbUfloat
            | Tf::Bc6hRgbSfloat
            | Tf::Bc7RgbaUnorm
            | Tf::Bc7RgbaUnormSrgb => unimplemented!(),
            Tf::Etc2RgbUnorm => (glow::COMPRESSED_RGB8_ETC2, glow::RGB, 0),
            Tf::Etc2RgbUnormSrgb => (glow::COMPRESSED_SRGB8_ETC2, glow::RGB, 0),
            Tf::Etc2RgbA1Unorm => (
                glow::COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2,
                glow::RGBA,
                0,
            ),
            Tf::Etc2RgbA1UnormSrgb => (
                glow::COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2,
                glow::RGBA,
                0,
            ),
            Tf::EacRUnorm => (glow::COMPRESSED_R11_EAC, glow::RED, 0),
            Tf::EacRSnorm => (glow::COMPRESSED_SIGNED_R11_EAC, glow::RED, 0),
            Tf::EacRgUnorm => (glow::COMPRESSED_RG11_EAC, glow::RG, 0),
            Tf::EacRgSnorm => (glow::COMPRESSED_SIGNED_RG11_EAC, glow::RG, 0),
            Tf::Astc4x4RgbaUnorm
            | Tf::Astc4x4RgbaUnormSrgb
            | Tf::Astc5x4RgbaUnorm
            | Tf::Astc5x4RgbaUnormSrgb
            | Tf::Astc5x5RgbaUnorm
            | Tf::Astc5x5RgbaUnormSrgb
            | Tf::Astc6x5RgbaUnorm
            | Tf::Astc6x5RgbaUnormSrgb
            | Tf::Astc6x6RgbaUnorm
            | Tf::Astc6x6RgbaUnormSrgb
            | Tf::Astc8x5RgbaUnorm
            | Tf::Astc8x5RgbaUnormSrgb
            | Tf::Astc8x6RgbaUnorm
            | Tf::Astc8x6RgbaUnormSrgb
            | Tf::Astc10x5RgbaUnorm
            | Tf::Astc10x5RgbaUnormSrgb
            | Tf::Astc10x6RgbaUnorm
            | Tf::Astc10x6RgbaUnormSrgb
            | Tf::Astc8x8RgbaUnorm
            | Tf::Astc8x8RgbaUnormSrgb
            | Tf::Astc10x8RgbaUnorm
            | Tf::Astc10x8RgbaUnormSrgb
            | Tf::Astc10x10RgbaUnorm
            | Tf::Astc10x10RgbaUnormSrgb
            | Tf::Astc12x10RgbaUnorm
            | Tf::Astc12x10RgbaUnormSrgb
            | Tf::Astc12x12RgbaUnorm
            | Tf::Astc12x12RgbaUnormSrgb => unimplemented!(),
        };

        super::TextureFormatDesc {
            internal,
            external,
            data_type,
        }
    }
}

pub(super) fn describe_vertex_format(vertex_format: wgt::VertexFormat) -> super::VertexFormatDesc {
    use super::VertexAttribKind as Vak;
    use wgt::VertexFormat as Vf;

    let (element_count, element_format, attrib_kind) = match vertex_format {
        Vf::Unorm8x2 => (2, glow::UNSIGNED_BYTE, Vak::Float),
        Vf::Snorm8x2 => (2, glow::BYTE, Vak::Float),
        Vf::Uint8x2 => (2, glow::UNSIGNED_BYTE, Vak::Integer),
        Vf::Sint8x2 => (2, glow::BYTE, Vak::Integer),
        Vf::Unorm8x4 => (4, glow::UNSIGNED_BYTE, Vak::Float),
        Vf::Snorm8x4 => (4, glow::BYTE, Vak::Float),
        Vf::Uint8x4 => (4, glow::UNSIGNED_BYTE, Vak::Integer),
        Vf::Sint8x4 => (4, glow::BYTE, Vak::Integer),
        Vf::Unorm16x2 => (2, glow::UNSIGNED_SHORT, Vak::Float),
        Vf::Snorm16x2 => (2, glow::SHORT, Vak::Float),
        Vf::Uint16x2 => (2, glow::UNSIGNED_SHORT, Vak::Integer),
        Vf::Sint16x2 => (2, glow::SHORT, Vak::Integer),
        Vf::Float16x2 => (2, glow::HALF_FLOAT, Vak::Float),
        Vf::Unorm16x4 => (4, glow::UNSIGNED_SHORT, Vak::Float),
        Vf::Snorm16x4 => (4, glow::SHORT, Vak::Float),
        Vf::Uint16x4 => (4, glow::UNSIGNED_SHORT, Vak::Integer),
        Vf::Sint16x4 => (4, glow::SHORT, Vak::Integer),
        Vf::Float16x4 => (4, glow::HALF_FLOAT, Vak::Float),
        Vf::Uint32 => (1, glow::UNSIGNED_INT, Vak::Integer),
        Vf::Sint32 => (1, glow::INT, Vak::Integer),
        Vf::Float32 => (1, glow::FLOAT, Vak::Float),
        Vf::Uint32x2 => (2, glow::UNSIGNED_INT, Vak::Integer),
        Vf::Sint32x2 => (2, glow::INT, Vak::Integer),
        Vf::Float32x2 => (2, glow::FLOAT, Vak::Float),
        Vf::Uint32x3 => (3, glow::UNSIGNED_INT, Vak::Integer),
        Vf::Sint32x3 => (3, glow::INT, Vak::Integer),
        Vf::Float32x3 => (3, glow::FLOAT, Vak::Float),
        Vf::Uint32x4 => (4, glow::UNSIGNED_INT, Vak::Integer),
        Vf::Sint32x4 => (4, glow::INT, Vak::Integer),
        Vf::Float32x4 => (4, glow::FLOAT, Vak::Float),
        Vf::Float64 | Vf::Float64x2 | Vf::Float64x3 | Vf::Float64x4 => unimplemented!(),
    };

    super::VertexFormatDesc {
        element_count,
        element_format,
        attrib_kind,
    }
}

pub fn map_filter_modes(
    min: wgt::FilterMode,
    mag: wgt::FilterMode,
    mip: wgt::FilterMode,
) -> (u32, u32) {
    use wgt::FilterMode as Fm;

    let mag_filter = match mag {
        Fm::Nearest => glow::NEAREST,
        Fm::Linear => glow::LINEAR,
    };

    let min_filter = match (min, mip) {
        (Fm::Nearest, Fm::Nearest) => glow::NEAREST_MIPMAP_NEAREST,
        (Fm::Nearest, Fm::Linear) => glow::NEAREST_MIPMAP_LINEAR,
        (Fm::Linear, Fm::Nearest) => glow::LINEAR_MIPMAP_NEAREST,
        (Fm::Linear, Fm::Linear) => glow::LINEAR_MIPMAP_LINEAR,
    };

    (min_filter, mag_filter)
}

pub fn map_address_mode(mode: wgt::AddressMode) -> u32 {
    match mode {
        wgt::AddressMode::Repeat => glow::REPEAT,
        wgt::AddressMode::MirrorRepeat => glow::MIRRORED_REPEAT,
        wgt::AddressMode::ClampToEdge => glow::CLAMP_TO_EDGE,
        wgt::AddressMode::ClampToBorder => glow::CLAMP_TO_BORDER,
        //wgt::AddressMode::MirrorClamp => glow::MIRROR_CLAMP_TO_EDGE,
    }
}

pub fn map_compare_func(fun: wgt::CompareFunction) -> u32 {
    use wgt::CompareFunction as Cf;
    match fun {
        Cf::Never => glow::NEVER,
        Cf::Less => glow::LESS,
        Cf::LessEqual => glow::LEQUAL,
        Cf::Equal => glow::EQUAL,
        Cf::GreaterEqual => glow::GEQUAL,
        Cf::Greater => glow::GREATER,
        Cf::NotEqual => glow::NOTEQUAL,
        Cf::Always => glow::ALWAYS,
    }
}

pub fn map_primitive_topology(topology: wgt::PrimitiveTopology) -> u32 {
    use wgt::PrimitiveTopology as Pt;
    match topology {
        Pt::PointList => glow::POINTS,
        Pt::LineList => glow::LINES,
        Pt::LineStrip => glow::LINE_STRIP,
        Pt::TriangleList => glow::TRIANGLES,
        Pt::TriangleStrip => glow::TRIANGLE_STRIP,
    }
}

pub(super) fn map_primitive_state(state: &wgt::PrimitiveState) -> super::PrimitiveState {
    //Note: state.polygon_mode is not supported, see `Features::NON_FILL_POLYGON_MODE`
    super::PrimitiveState {
        //Note: we are flipping the front face, so that
        // the Y-flip in the generated GLSL keeps the same visibility.
        // See `naga::back::glsl::WriterFlags::ADJUST_COORDINATE_SPACE`.
        front_face: match state.front_face {
            wgt::FrontFace::Cw => glow::CCW,
            wgt::FrontFace::Ccw => glow::CW,
        },
        cull_face: match state.cull_mode {
            Some(wgt::Face::Front) => glow::FRONT,
            Some(wgt::Face::Back) => glow::BACK,
            None => 0,
        },
        clamp_depth: state.clamp_depth,
    }
}

pub fn _map_view_dimension(dim: wgt::TextureViewDimension) -> u32 {
    use wgt::TextureViewDimension as Tvd;
    match dim {
        Tvd::D1 | Tvd::D2 => glow::TEXTURE_2D,
        Tvd::D2Array => glow::TEXTURE_2D_ARRAY,
        Tvd::Cube => glow::TEXTURE_CUBE_MAP,
        Tvd::CubeArray => glow::TEXTURE_CUBE_MAP_ARRAY,
        Tvd::D3 => glow::TEXTURE_3D,
    }
}

fn map_stencil_op(operation: wgt::StencilOperation) -> u32 {
    use wgt::StencilOperation as So;
    match operation {
        So::Keep => glow::KEEP,
        So::Zero => glow::ZERO,
        So::Replace => glow::REPLACE,
        So::Invert => glow::INVERT,
        So::IncrementClamp => glow::INCR,
        So::DecrementClamp => glow::DECR,
        So::IncrementWrap => glow::INCR_WRAP,
        So::DecrementWrap => glow::DECR_WRAP,
    }
}

fn map_stencil_ops(face: &wgt::StencilFaceState) -> super::StencilOps {
    super::StencilOps {
        pass: map_stencil_op(face.pass_op),
        fail: map_stencil_op(face.fail_op),
        depth_fail: map_stencil_op(face.depth_fail_op),
    }
}

pub(super) fn map_stencil(state: &wgt::StencilState) -> super::StencilState {
    super::StencilState {
        front: super::StencilSide {
            function: map_compare_func(state.front.compare),
            mask_read: state.read_mask,
            mask_write: state.write_mask,
            reference: 0,
            ops: map_stencil_ops(&state.front),
        },
        back: super::StencilSide {
            function: map_compare_func(state.back.compare),
            mask_read: state.read_mask,
            mask_write: state.write_mask,
            reference: 0,
            ops: map_stencil_ops(&state.back),
        },
    }
}

fn map_blend_factor(factor: wgt::BlendFactor) -> u32 {
    use wgt::BlendFactor as Bf;
    match factor {
        Bf::Zero => glow::ZERO,
        Bf::One => glow::ONE,
        Bf::Src => glow::SRC_COLOR,
        Bf::OneMinusSrc => glow::ONE_MINUS_SRC_COLOR,
        Bf::Dst => glow::DST_COLOR,
        Bf::OneMinusDst => glow::ONE_MINUS_DST_COLOR,
        Bf::SrcAlpha => glow::SRC_ALPHA,
        Bf::OneMinusSrcAlpha => glow::ONE_MINUS_SRC_ALPHA,
        Bf::DstAlpha => glow::DST_ALPHA,
        Bf::OneMinusDstAlpha => glow::ONE_MINUS_DST_ALPHA,
        Bf::Constant => glow::CONSTANT_COLOR,
        Bf::OneMinusConstant => glow::ONE_MINUS_CONSTANT_COLOR,
        Bf::SrcAlphaSaturated => glow::SRC_ALPHA_SATURATE,
    }
}

fn map_blend_component(component: &wgt::BlendComponent) -> super::BlendComponent {
    super::BlendComponent {
        src: map_blend_factor(component.src_factor),
        dst: map_blend_factor(component.dst_factor),
        equation: match component.operation {
            wgt::BlendOperation::Add => glow::FUNC_ADD,
            wgt::BlendOperation::Subtract => glow::FUNC_SUBTRACT,
            wgt::BlendOperation::ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            wgt::BlendOperation::Min => glow::MIN,
            wgt::BlendOperation::Max => glow::MAX,
        },
    }
}

pub(super) fn map_blend(blend: &wgt::BlendState) -> super::BlendDesc {
    super::BlendDesc {
        color: map_blend_component(&blend.color),
        alpha: map_blend_component(&blend.alpha),
    }
}

pub(super) fn map_storage_access(access: wgt::StorageTextureAccess) -> u32 {
    match access {
        wgt::StorageTextureAccess::ReadOnly => glow::READ_ONLY,
        wgt::StorageTextureAccess::WriteOnly => glow::WRITE_ONLY,
        wgt::StorageTextureAccess::ReadWrite => glow::READ_WRITE,
    }
}

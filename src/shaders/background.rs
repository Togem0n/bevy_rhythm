use super::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::reflect::TypeUuid;

pub struct Background;
pub fn setup_background(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    window: WindowDescriptor,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            include_str!("bg.vert"),
            ShaderStage::Vertex,
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            include_str!("bg.frag"),
            ShaderStage::Fragment,
        ))),
    }));

    commands
        .spawn_bundle(SpriteBundle {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(
                window.width + 10.,
                window.height + 10.,
                1.,
            )),
            ..Default::default()
        })
        .insert(Background)
        .insert(ShaderInputs {
            time: 0.,
            resolution: Vec2::new(window.width / window.height, 1.),
        });
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "ccbc87c0-6917-11ed-9022-0242ac120002"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
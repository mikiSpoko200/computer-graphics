#[macro_use]
extern crate glium;
mod teapot;

use teapot::Vertex;

use glam::Mat4;
use glium::Surface;


fn main() {
    use glium::glutin;

    println!("Creating window with glutin");
    let event_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(800, 800))
        .with_title("Super labyrinth 3D")
        .with_max_inner_size(glutin::dpi::LogicalSize::new(800, 800))
        .with_min_inner_size(glutin::dpi::LogicalSize::new(800, 800));
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &event_loop).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

     let cb = glutin::ContextBuilder::new().with_depth_buffer(24);

    let model = [
        [0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let mut t: f32 = 0.0;

    let program = program!(&display,
        330 => {
            vertex: r#"
                #version 330
                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;
                uniform mat4 transformation;

                void main() {
                    v_normal = transpose(inverse(mat3(transformation))) * normal;
                    gl_Position = transformation * vec4(position, 1.0);
                }
            "#,
            fragment: r#"
                #version 330

                in vec3 v_normal;
                out vec4 color;
                uniform vec3 u_light;

                void main() {
                    float brightness = dot(normalize(v_normal), normalize(u_light));
                    vec3 dark_color = vec3(0.6, 0.0, 0.0);
                    vec3 regular_color = vec3(1.0, 0.0, 0.0);
                    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
                }
            "#
        }
    ).unwrap();
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    event_loop.run(move |ev, _, control_flow| {

        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        t += 0.02;
        if t > std::f32::consts::PI * 2.0 {
            t = 0.0;
        }
        let rotation = [
            [ t.cos(), 0.0, t.sin(), 0.0],
            [0.0,      1.0, 0.0,     0.0],
            [-t.sin(), 0.0, t.cos(), 0.0],
            [0.0,      0.0, 0.0,     1.0f32],
        ];

        let rot_matrix = glam::Mat4::from_cols_array_2d(&rotation);
        let model_matrix = glam::Mat4::from_cols_array_2d(&model);

        let aggregated = rot_matrix.mul_mat4(&model_matrix);
        let transformation = aggregated.to_cols_array_2d();

        let light = [-1.0, 0.4, 0.9f32];
        let mut frame = display.draw();
        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let uniforms = uniform!(
            transformation: transformation,
            u_light: light
        );
        frame.draw((&positions, &normals), &indices, &program, &uniforms, &params).unwrap();
        frame.finish().unwrap();
    });
}

#[macro_use]
extern crate glium;

use std::io;
use std::io::prelude::*;
use glium::Surface;

#[derive(Copy,Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[derive(Debug)]
enum Command {
    Pen(u16),
    Up,
    Down,
    North(u32),
    South(u32),
    East(u32),
    West(u32),
}

fn process_command(command: String) -> Command {
    let mut split = command.split(' ');
    let elem: &str = split.next().expect("No elem omg");
    match split.next() {
        None => {
            match elem.as_ref() {
                "U" => Command::Up,
                "D" => Command::Down,
                _ => panic!("Invalid command")
            }
        }
        Some(param) => {
            let param: u32 = match param.parse() {
                Ok(num) => num,
                Err(_) => panic!("Param should be a valid integer"),
            };
            match elem.as_ref() {
                "P" => Command::Pen(param as u16),
                "N" => Command::North(param),
                "S" => Command::South(param),
                "E" => Command::East(param),
                "W" => Command::West(param),
                _ => panic!("Invalid command")
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let commands: Vec<Command> = stdin.lock().lines().map(|line| {
        let line = line.expect("WTF");
        process_command(line)
    }).collect();

    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }

    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }

    "#;

    let mut vertices = Vec::new();
    let mut current_pos = Vertex { position: [0.0, 0.0] };
    let mut pen_down = false;
    for command in commands {
        match command {
            Command::Up => pen_down = false,
            Command::Down => pen_down = true,
            Command::North(_) | Command::South(_) | Command::East(_) | Command::West(_) => {
                let (dx, dy): (i64, i64) = match command {
                    Command::North(dist) => (0, -(dist as i64)),
                    Command::South(dist) => (0, dist as i64),
                    Command::East(dist) => (dist as i64, 0),
                    Command::West(dist) => (-(dist as i64), 0),
                    _ => panic!("Cannot happen"),
                };
                let new_pos = Vertex {
                    position: [
                        current_pos.position[0] + (dx as f32) * 0.01,
                        current_pos.position[1] + (dy as f32) * 0.01
                    ]
                };
                if pen_down {
                    vertices.push(current_pos);
                    vertices.push(new_pos);
                }
                current_pos = new_pos;
            },
            _ => ()//panic!("not yet implemented")
        }
    }

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::LinesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
        // listing the events produced by the window and waiting to be received
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

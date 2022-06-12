use wgpu_tetris::run;

fn main() {
    pollster::block_on(run());
}
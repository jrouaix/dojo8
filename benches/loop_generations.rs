use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gameoflife::grid::Grid;

fn run_generations(mut grid: Grid) {
  for _ in 0..1_000 {
    grid = grid.next();
  }
}

pub fn criterion_benchmark(c: &mut Criterion) {
  let grid = Grid::random();
  c.bench_function("loop generations", |b| b.iter(|| run_generations(black_box(grid.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

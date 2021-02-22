use bevy::ecs::{IntoSystem,World,SystemStage,Resources,Query,Stage};
use criterion::{criterion_group, criterion_main, Criterion};

criterion_group!(benches, empty_systems, busy_systems, contrived);
criterion_main!(benches);

fn run_stage(stage: &mut SystemStage, world: &mut World, resources: &mut Resources) {
    // !!NB!! Uncomment next line when running with old executor.
    //stage.initialize(world, resources);
    stage.run(world, resources);
}

struct A(f32);
struct B(f32);
struct C(f32);
struct D(f32);
struct E(f32);

const ENTITY_BUNCH: usize = 5000;

fn empty_systems(criterion: &mut Criterion) {
    let mut world = World::new();
    let mut resources = Resources::default();
    let mut group = criterion.benchmark_group("empty_systems");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    fn empty() {}
    for amount in 0..5 {
        let mut stage = SystemStage::parallel();
        for _ in 0..amount {
            stage.add_system(empty.system());
        }
        run_stage(&mut stage, &mut world, &mut resources);
        group.bench_function(&format!("{:03}_systems", amount), |bencher| {
            bencher.iter(|| {
                run_stage(&mut stage, &mut world, &mut resources);
            });
        });
    }
    for amount in 1..21 {
        let mut stage = SystemStage::parallel();
        for _ in 0..amount {
            stage
                .add_system(empty.system())
                .add_system(empty.system())
                .add_system(empty.system())
                .add_system(empty.system())
                .add_system(empty.system());
        }
        run_stage(&mut stage, &mut world, &mut resources);
        group.bench_function(&format!("{:03}_systems", 5 * amount), |bencher| {
            bencher.iter(|| {
                run_stage(&mut stage, &mut world, &mut resources);
            });
        });
    }
    group.finish()
}

fn busy_systems(criterion: &mut Criterion) {
    fn ab(mut q: Query<(&mut A, &mut B)>) {
        for (mut a, mut b) in q.iter_mut() {
            std::mem::swap(&mut a.0, &mut b.0);
        }
    }
    fn cd(mut q: Query<(&mut C, &mut D)>) {
        for (mut c, mut d) in q.iter_mut() {
            std::mem::swap(&mut c.0, &mut d.0);
        }
    }
    fn ce(mut q: Query<(&mut C, &mut E)>) {
        for (mut c, mut e) in q.iter_mut() {
            std::mem::swap(&mut c.0, &mut e.0);
        }
    }
    let mut world = World::new();
    let mut resources = Resources::default();
    let mut group = criterion.benchmark_group("busy_systems");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    for entity_bunches in 1..6 {
        world.spawn_batch((0..4 * ENTITY_BUNCH).map(|_| (A(0.0), B(0.0))));
        world.spawn_batch((0..4 * ENTITY_BUNCH).map(|_| (A(0.0), B(0.0), C(0.0))));
        world.spawn_batch((0..ENTITY_BUNCH).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.spawn_batch((0..ENTITY_BUNCH).map(|_| (A(0.0), B(0.0), C(0.0), E(0.0))));
        for system_amount in 0..5 {
            let mut stage = SystemStage::parallel();
            stage
                .add_system(ab.system())
                .add_system(cd.system())
                .add_system(ce.system());
            for _ in 0..system_amount {
                stage
                    .add_system(ab.system())
                    .add_system(cd.system())
                    .add_system(ce.system());
            }
            run_stage(&mut stage, &mut world, &mut resources);
            group.bench_function(
                &format!(
                    "{:02}x_entities_{:02}_systems",
                    entity_bunches,
                    3 * system_amount + 3
                ),
                |bencher| {
                    bencher.iter(|| {
                        run_stage(&mut stage, &mut world, &mut resources);
                    });
                },
            );
        }
    }
    group.finish()
}

fn contrived(criterion: &mut Criterion) {
    fn s_0(mut q_0: Query<(&mut A, &mut B)>) {
        for (mut c_0, mut c_1) in q_0.iter_mut() {
            std::mem::swap(&mut c_0.0, &mut c_1.0);
        }
    }
    fn s_1(mut q_0: Query<(&mut A, &mut C)>, mut q_1: Query<(&mut B, &mut D)>) {
        for (mut c_0, mut c_1) in q_0.iter_mut() {
            std::mem::swap(&mut c_0.0, &mut c_1.0);
        }
        for (mut c_0, mut c_1) in q_1.iter_mut() {
            std::mem::swap(&mut c_0.0, &mut c_1.0);
        }
    }
    fn s_2(mut q_0: Query<(&mut C, &mut D)>) {
        for (mut c_0, mut c_1) in q_0.iter_mut() {
            std::mem::swap(&mut c_0.0, &mut c_1.0);
        }
    }
    let mut world = World::new();
    let mut resources = Resources::default();
    let mut group = criterion.benchmark_group("contrived");
    group.warm_up_time(std::time::Duration::from_millis(500));
    group.measurement_time(std::time::Duration::from_secs(3));
    for entity_bunches in 1..6 {
        world.spawn_batch((0..ENTITY_BUNCH).map(|_| (A(0.0), B(0.0), C(0.0), D(0.0))));
        world.spawn_batch((0..ENTITY_BUNCH).map(|_| (A(0.0), B(0.0))));
        world.spawn_batch((0..ENTITY_BUNCH).map(|_| (C(0.0), D(0.0))));
        for system_amount in 0..5 {
            let mut stage = SystemStage::parallel();
            stage
                .add_system(s_0.system())
                .add_system(s_1.system())
                .add_system(s_2.system());
            for _ in 0..system_amount {
                stage
                    .add_system(s_0.system())
                    .add_system(s_1.system())
                    .add_system(s_2.system());
            }
            run_stage(&mut stage, &mut world, &mut resources);
            group.bench_function(
                &format!(
                    "{:02}x_entities_{:02}_systems",
                    entity_bunches,
                    3 * system_amount + 3
                ),
                |bencher| {
                    bencher.iter(|| {
                        run_stage(&mut stage, &mut world, &mut resources);
                    });
                },
            );
        }
    }
    group.finish()
}
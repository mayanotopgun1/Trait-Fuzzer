pub trait CSpace<const N: usize> {
    type Traj;
}

pub struct Const<const R: usize>;

pub trait Obstacle<CS, const N: usize> {
    fn trajectory_free<FT, S1>(&self, t: &FT)
    where
        CS::Traj: Sized,
        CS: CSpace<N>;
}

pub trait ExtendedObstacle<CS, const N: usize>: Obstacle<CS, N>
where
    CS: CSpace<N>,
{
    fn extended_trajectory_free<TF, S1>(&self, t: &TF) -> bool
    where
        CS::Traj: Sized,
    {
        self.trajectory_free::<TF, S1>(t);
        true
    }
}

impl<CS> ExtendedObstacle<CS, N> for ObstacleSpace2df32
where
    CS: CSpace<N>,
{
}

const N: usize = 4;

struct ObstacleSpace2df32;

impl<CS> Obstacle<CS, N> for ObstacleSpace2df32 {
    fn trajectory_free<TF, S1>(&self, _t: &TF)
    where
        CS::Traj: Sized,
        CS: CSpace<N>,
    {
    }
}

fn main() {}
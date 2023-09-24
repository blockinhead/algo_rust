#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn flip(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Lift {
    capacity: u32,
    direction: Direction,
    on_board: Vec<u32>,
}

impl Lift {
    pub fn new(capacity: u32, direction: Direction, on_board: Vec<u32>) -> Self {
        Self {
            capacity,
            direction,
            on_board,
        }
    }
}

#[derive(Debug)]
struct FloorPassengeres {
    goes_up: Vec<u32>,
    goes_down: Vec<u32>
}

impl FloorPassengeres {
    pub fn init(passangers: &[Vec<u32>]) -> Vec<Self> {
        let mut res = Vec::new();
        for (i, q) in passangers.iter().enumerate() {
            let mut up = Vec::new();
            let mut down = Vec::new();
            for p in q.iter() {
                if p > &(i as u32) {up.push(*p)}
                else { down.push(*p) }
            }
            res.push(Self {goes_up: up, goes_down: down})
        }

        res
    }

    pub fn no_one_goes_this_direction(&self, direction: Direction) -> bool {
        if direction == Direction::Up && self.goes_up.is_empty() { return true }
        if direction == Direction::Down && self.goes_down.is_empty() { return true }
        return false
    }

    pub fn is_no_one_here(&self) -> bool {
        return  self.goes_down.is_empty() && self.goes_up.is_empty()
    }
}

fn are_queues_empty(passengers: &Vec<FloorPassengeres>) -> bool {
    for i in 0..passengers.len() {
        if !passengers[i].is_no_one_here() { return false }
    }
    return true
}

fn the_lift(queues: &[Vec<u32>], capacity: u32) -> Vec<u32> {
    let mut res = vec![0];
    let mut lift = Lift::new(capacity, Direction::Up, Vec::new());
    let passangeres = FloorPassengeres::init(queues);

    dbg!(passangeres);

    // while !lift.on_board.is_empty() && !are_queues_empty(&passangeres) {
    //
    // }

    res
}

pub fn te_the_lift() {
    assert_eq!(the_lift(&[vec![], vec![], vec![5,5,5],vec![],vec![],vec![],vec![]], 5), &[0, 2, 5, 0]);
    assert_eq!(the_lift(&[vec![],vec![],vec![1],vec![],vec![],vec![],vec![]], 5), &[0, 2, 1, 0]);
    assert_eq!(the_lift(&[vec![],vec![3],vec![4],vec![],vec![5],vec![],vec![]], 5), &[0, 1, 2, 3, 4, 5, 0]);
    assert_eq!(the_lift(&[vec![],vec![0],vec![],vec![],vec![2],vec![3],vec![]], 5), &[0, 5, 4, 3, 2, 1, 0]);
}
/// I went to the woods because I wished to live deliberately, to front only the essential facts of
/// life, and see if I could not learn what it had to teach, and not, when I came to die, discover 
/// that I had not lived. I did not wish to live what was not life, living is so dear; nor did I 
/// wish to practise resignation, unless it was quite necessary. I wanted to live deep and suck out
/// all the marrow of life, to live so sturdily and Spartan-like as to put to rout all that was not
/// life, to cut a broad swath and shave close, to drive life into a corner, and reduce it to its
/// lowest terms. - Henry David Thoreau

extern crate rand;
extern crate mursten;
extern crate mursten_ggez_backend;
extern crate nalgebra;
extern crate petgraph;
extern crate ggez;

use mursten::graphics::{Draw, Graphics, DrawPrimitives, Color};
use mursten::logic::{Update, ElapsedDelta};
use mursten::input::{JoystickProvider, Dpad};
use petgraph::stable_graph::{StableGraph, NodeIndex};
use std::ops::{Index, IndexMut};
use nalgebra::*;

use entities::*;
use ui::*;


struct World {
    player_id: NodeIndex,
    content: StableGraph<Entity, Edge>,
    spawn_cooldown: f32,
    speed: f32,
    delta: f32,
}

impl Index<NodeIndex> for World {
    type Output = Entity;

    fn index(&self, id: NodeIndex) -> &Entity {
        &self.content[id]
    }
}

impl IndexMut<NodeIndex> for World {
    fn index_mut(&mut self, id: NodeIndex) -> &mut Entity {
        &mut self.content[id]
    }
}

impl ElapsedDelta for World {
    fn delta(&self) -> f32 {
        self.delta
    }
}

#[derive(Clone)]
struct Edge;


mod entities {
    use super::*;
    
    #[derive(Clone)]
    pub struct Entity {
        pub kind: EntityKind,
        pub position: Point2<f32>,
    }
    
    impl Entity { 
        pub fn new(kind: EntityKind, position: Point2<f32>) -> Self {
            Self { kind, position }
        }
    }

    #[derive(Clone, Debug)]
    pub enum EntityKind {
        Player(Player),
        Bag(Inventory),
        Tent(Inventory),
        Grass,
        Stone,
        Tree,
        Bush,
        Axe,
        Pond(f32),
        //Monster(Monster),
    }

    // #[derive(Clone, Debug)]
    // pub enum Monster {
    //     Mouse,
    // }
    
    use mursten::graphics::{Draw, DrawPrimitives, DrawMode, PushTransform};

    impl<S> Draw<S> for Entity
    where
        S: DrawPrimitives,
    {
        fn draw(&self, surface: &mut S) {
            let mut s = PushTransform::new(surface, convert(Translation2::from(self.position.coords)));
            self.kind.draw(&mut s);
        }
    }

    impl<S> Draw<S> for EntityKind
    where
        S: DrawPrimitives
    {
        fn draw(&self, surface: &mut S) {
            match self {
                EntityKind::Player(Player { t, log_speed, ..}) => {

                    let amp = (log_speed.norm() - 1.0).max(0.0).min(1.0);
                    let lamp = amp * 0.5 + 0.5;
                    let speed = 6.0;
                    let t = t * speed;
                    
                    
                    let xoff = log_speed.x * 0.7;
                    
                    
                    let mut surface = PushTransform::new(surface, convert(Similarity2::from_scaling(3.0)));
                    
                    surface.set_color(Palette::Player);
                    
                    // Left leg
                    surface.rectangle(DrawMode::Fill, Point2::new(-3.0,  -4.0) + Vector2::y() * (t).sin() * 1.0 * amp, 2.0, 4.0);
                    
                    // Right leg
                    surface.rectangle(DrawMode::Fill, Point2::new( 1.0,  -4.0) - Vector2::y() * (t).sin() * 1.0 * amp, 2.0, 4.0);
                    
                    // Torso
                    surface.rectangle(DrawMode::Fill, Point2::new(-3.0, -10.0) + Vector2::y() * (2.0*t - 0.1).sin() * 0.7 * lamp, 6.0, 8.0);
                    
                    // Head
                    surface.set_color(Palette::PlayerSkin);
                    surface.rectangle(DrawMode::Fill, Point2::new(-4.0, -16.0) + Vector2::y() * (2.0*t - 0.2).sin() * 0.5 * lamp + Vector2::x() * xoff * 0.5, 8.0, 8.0);
                    
                    
                    if log_speed.y >= 0.0 {
                        // Eyes
                        surface.set_color(Palette::PlayerEyes);
                        surface.rectangle(DrawMode::Fill, Point2::new(-3.0, -14.0) + Vector2::y() * (2.0*t - 0.2).sin() * 0.6 * lamp + Vector2::x() * xoff * 0.9, 2.0, 1.0);
                        surface.rectangle(DrawMode::Fill, Point2::new( 1.0, -14.0) + Vector2::y() * (2.0*t - 0.2).sin() * 0.6 * lamp + Vector2::x() * xoff * 0.9, 2.0, 1.0);

                        // Beard
                        surface.set_color(Palette::PlayerHair);
                        surface.rectangle(DrawMode::Fill, Point2::new(-3.0, -12.0) + Vector2::y() * (2.0*t - 0.25).sin() * 0.6 * lamp + Vector2::x() * xoff, 6.0, 4.0);
                        surface.set_color(Palette::PlayerSkin);
                        surface.rectangle(DrawMode::Fill, Point2::new(-1.0, -10.0) + Vector2::y() * (2.0*t - 0.25).sin() * 0.6 * lamp + Vector2::x() * xoff, 2.0, 1.0);
                    }
                    
                    // surface.rectangle(DrawMode::Fill, Point2::new(), ,);
                },
                EntityKind::Bag(..) => {
                    surface.set_color(Palette::Bag);
                    surface.polygon(DrawMode::Fill, &vec![
                        Point2::new(-10.0, -10.0),
                        Point2::new(-10.0,  10.0),
                        Point2::new( 10.0,  10.0),
                        Point2::new( 10.0, -10.0)
                    ]);
                },
                EntityKind::Tent(..) => {
                    surface.set_color(Palette::Tent);
                    surface.polygon(DrawMode::Fill, &vec![
                        Point2::new(-10.0, 0.0),
                        Point2::new(-10.0, 10.0),
                        Point2::new(0.0, 20.0),
                        Point2::new(10.0, 10.0),
                        Point2::new(10.0, 0.0),
                        Point2::new(0.0, -5.0)
                    ]);
                },
                // EntityKind::Monster(monster) => {
                //     monster.draw(surface);
                // },
                EntityKind::Pond(size) => {
                    surface.set_color(Palette::Water);
                    surface.circle(DrawMode::Fill, Point2::origin(), *size)
                },
                _ => {
                    surface.set_color(Palette::Unknown);
                    surface.circle(DrawMode::Fill, Point2::origin(), 5.0)
                },
            }
        }
    }

    // #[derive(Clone, Copy, Debug)]
    // enum MonsterPalette {
    //     MouseSkin,
    //     MousePink,
    //     MouseBrown,
    //     Black,
    //     Light,
    // }
    // 
    // impl Color for MonsterPalette {
    //     fn into_rgba(self) -> [f32; 4] {
    //         match self {
    //             MonsterPalette::MouseSkin => [0.80, 0.80, 0.80, 1.0],
    //             MonsterPalette::MousePink => [0.80, 0.10, 0.40, 1.0],
    //             MonsterPalette::MouseBrown => [0.70, 0.70, 0.20, 1.0],
    //             MonsterPalette::Black => [0.10, 0.10, 0.10, 1.0],
    //             MonsterPalette::Light => [0.90, 0.90, 0.90, 1.0],
    //         }
    //     }
    // }


    // impl<S> Draw<S> for Monster
    //     where
    //         S: DrawPrimitives
    // {
    //     fn draw(&self, surface: &mut S) {
    //         match self {
    //             Monster::Mouse => {
    //                 // Left ear
    //                 surface.set_color(MonsterPalette::MouseSkin);
    //                 surface.circle(DrawMode::Fill, Point2::new(-24.0, -20.0), 15.0);
    //                 surface.set_color(MonsterPalette::MousePink);
    //                 surface.circle(DrawMode::Fill, Point2::new(-27.0, -18.0), 10.0);
    //                 
    //                 // Right ear
    //                 surface.set_color(MonsterPalette::MouseSkin);
    //                 surface.circle(DrawMode::Fill, Point2::new(24.0, -20.0), 15.0);
    //                 surface.set_color(MonsterPalette::MousePink);
    //                 surface.circle(DrawMode::Fill, Point2::new(27.0, -18.0), 10.0);
    //                 
    //                 // Body
    //                 surface.set_color(MonsterPalette::MouseSkin);
    //                 surface.ellipse(DrawMode::Fill, Point2::origin(), 26.0, 22.0);
    //                 
    //                 // Eyes
    //                 surface.set_color(MonsterPalette::Black);
    //                 surface.circle(DrawMode::Fill, Point2::new(-20.0, 2.0), 7.0);
    //                 surface.circle(DrawMode::Fill, Point2::new(20.0, 2.0), 7.0);
    //                 surface.set_color(MonsterPalette::Light);
    //                 surface.square_centered(DrawMode::Fill, Point2::new(-21.0, 2.0), 5.0);
    //                 surface.square_centered(DrawMode::Fill, Point2::new(19.0, 2.0), 5.0);
    //             }
    //         }
    //     }
    // }
    
    impl EntityKind {
        pub fn get_item(&self) -> Option<Item> {
            match self {
                EntityKind::Axe => Some(Item::Axe),
                EntityKind::Bush => Some(Item::Berry),
                _ => None,
            }
        }
        
        pub fn is_pickupable(&self) -> bool {
            self.get_item().is_some()
        }
        
        pub fn size(&self) -> f32 {
            match self {
                EntityKind::Pond(size) => *size,
                _ => 10.0,
            }
        }
        
        pub fn is_solid(&self) -> bool {
            match self {
                EntityKind::Bag(_) => true,
                EntityKind::Tent(_) => true,
                EntityKind::Stone => true,
                EntityKind::Tree => true,
                EntityKind::Bush => true,
                EntityKind::Pond(_) => true,
                
                EntityKind::Grass => false,
                EntityKind::Player(_) => false,
                EntityKind::Axe => false,
                _ => false,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum Item {
        Axe,
        Berry,
        Bottle(bool),
    }
    
    impl Item {
        pub fn do_action(self, world: &mut World) -> Option<Self> {
            match self {
                Item::Bottle(full) => {
                    if full {
                        eprintln!("Drinking water");
                        world.get_player_mut().thirst = 1.0;
                        Some(Item::Bottle(false))
                    }
                    else if world.water_in_front_of_player() {
                        eprintln!("Filling bottle");
                        Some(Item::Bottle(true))
                    }
                    else {
                        eprintln!("The bottle is empty!");
                        Some(Item::Bottle(false))
                    }
                },
                e => Some(e),
            }
        }
        
        pub fn action_tooltip(self, world: &World) -> Option<Text> {
            match self {
                Item::Bottle(full) => {
                    if full {
                        Some(Text::DrinkBottle)
                    }
                    else if world.water_in_front_of_player() {
                        Some(Text::FillBottle)
                    }
                    else {
                        None
                    }
                },
                _ => None
            }
        }
        
    }
    
    impl<S> Draw<S> for Item
        where
            S: DrawPrimitives
    {
        fn draw(&self, surface: &mut S) {
            match self {
                Item::Axe => {
                    surface.set_color(Palette::Unknown);
                    surface.circle(DrawMode::Fill, Point2::origin(), 5.0);
                },
                Item::Berry => {
                    surface.set_color(Palette::Unknown);
                    surface.circle(DrawMode::Fill, Point2::origin(), 5.0);
                },
                Item::Bottle(full) => {
                    let mut surface = PushTransform::new(surface, convert(Similarity2::from_scaling(2.0)));
                    
                    surface.set_color(Palette::Glass);
                    surface.polygon(DrawMode::Fill, &vec![
                        Point2::new(2.0, -6.0),
                        Point2::new(2.0, -4.0),
                        Point2::new(5.0, -3.0),
                        Point2::new(5.0, 6.0),
                        Point2::new(-5.0, 6.0),
                        Point2::new(-5.0, -3.0),
                        Point2::new(-2.0, -4.0),
                        Point2::new(-2.0, -6.0),
                        Point2::new(-2.0, -6.0),
                    ]);
                    if *full {
                        surface.set_color(Palette::Water);
                        surface.rectangle(DrawMode::Fill, Point2::new(-4.0, -3.0), 8.0, 8.0);
                    }
                }
            }
        }
    }
    
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct Player {
        pub hands: HashMap<Dpad, Item>,
        pub current_hand: Dpad,
        pub log_speed: Vector2<f32>,
        pub t: f32,
        pub thirst: f32,
        pub hunger: f32,
        pub sleep: f32,
    }
    
    impl Player {
        pub fn new() -> Self {
            let mut n = Self {
                hands: HashMap::new(),
                current_hand: Dpad::Up,
                log_speed: Vector2::new(0.0, -1.0),
                t: 0.0,
                thirst: 1.0,
                hunger: 1.0,
                sleep: 1.0,
            };
            n.hands.insert(Dpad::Left, Item::Bottle(true));
            n
        }
    }

    #[derive(Clone, Debug)]
    pub struct Inventory {
    }
}

mod ui {
    use super::*;
    use mursten::graphics::{Draw, DrawPrimitives, DrawMode, PushTransform};

    #[derive(Clone, Debug)]
    pub struct Button {
        is_a: bool,
        pub pressed: bool,
    }
    
    impl Button {
        pub fn a() -> Self {
            Self { is_a: true, pressed: false }
        }
        pub fn b() -> Self {
            Self { is_a: false, pressed: false }
        }
    }

    impl<S> Draw<S> for Button
    where
        S: DrawPrimitives
    {
        fn draw(&self, surface: &mut S) {
            let color = if self.is_a { UIPalette::ButtonA(self.pressed) } else { UIPalette::ButtonB(self.pressed) };
            let text = if self.is_a { Text::ButtonA } else { Text::ButtonB };
            surface.set_color(color);
            surface.circle(DrawMode::Fill, Point2::origin(), 10.0);
            surface.set_color(UIPalette::Text);
            surface.text(Point2::origin() - text.center(), text.str());
        }
    }

    #[derive(Clone, Debug)]
    pub enum StatIndicator {
        Sleep(Player),
        Thirst(Player),
        Hunger(Player),
    }
    
    impl StatIndicator {
        pub fn stat(&self) -> f32 {
            match self {
                StatIndicator::Sleep(p) => p.sleep,
                StatIndicator::Thirst(p) => p.thirst,
                StatIndicator::Hunger(p) => p.hunger,
            }
        }
        
        pub fn color(&self) -> UIPalette {
            match self {
                StatIndicator::Sleep(_) => UIPalette::StatIndicatorSleep,
                StatIndicator::Thirst(_) => UIPalette::StatIndicatorThirst,
                StatIndicator::Hunger(_) => UIPalette::StatIndicatorHunger,
            }
        }
        
        pub fn set_player(&mut self, player: &Player) {
            match self {
                StatIndicator::Sleep(ref mut p) => p.clone_from(player),
                StatIndicator::Thirst(ref mut p) => p.clone_from(player),
                StatIndicator::Hunger(ref mut p) => p.clone_from(player),
            }
        }
    }
    
    impl<S> Draw<S> for StatIndicator
        where
            S: DrawPrimitives
    {
        fn draw(&self, surface: &mut S) {
            let size = 15.0;
            let points = 30.0;

            surface.set_color(UIPalette::StatIndicatorBack);
            surface.circle(DrawMode::Fill, Point2::origin(), size + 2.0);
            surface.set_color(self.color());
            indicator(surface, size, self.stat());
        }
    }
    
    pub fn indicator<S: DrawPrimitives>(s: &mut S, radius: f32, l: f32) {
        let len = 20;
        let points = (0..(len as f32 * l).round() as i32)
            .map(|i| {
                Rotation2::new(f32::two_pi() / len as f32).powf(i as f32) * Point2::new(0.0, radius)
            });
        let points = Some(Point2::origin()).iter().cloned().chain(points).collect();
        s.polygon(DrawMode::Fill, &points);
    }

    #[derive(Clone, Debug)]
    pub struct Selector {
        axis: Vector2<f32>,
        pub choice: Option<Dpad>,
        pub player: Player,
        pub state: SelectorState,
    }
    
    impl Selector {
        pub fn new(player: Player) -> Self {
            Self {
                player,
                axis: Vector2::new(0.0, 0.0),
                choice: None,
                state: SelectorState::Idle,
            }
        }
        pub fn is_visible(&self) -> bool {
            self.state == SelectorState::AboutToCancel || self.state == SelectorState::ItemChosed
        }
    }
    
    impl<C> Update<C> for Selector
    where
        C: ElapsedDelta,
    {
        fn update(&mut self, context: &mut C) {
            
            let direction = if let Some(dpad) = self.choice {
                dpad.into()
            }
            else {
                Vector2::new(0.0, 0.0)
            };
            let d = direction - self.axis;
            self.axis = self.axis + d / 2.0;
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum SelectorState {
        Idle,
        Deciding(f32),
        ItemChosed,
        AboutToCancel,
    }
    
    impl<S> Draw<S> for Selector
    where
        S: DrawPrimitives
    {
        fn draw(&self, surface: &mut S) {
            let size = 30.0;
            let offset = size * 1.2;
            let circle_size = 20.0;
            let circle_pos = Point2::origin() + self.axis * offset;

            surface.set_color(UIPalette::SelectorBack);
            surface.circle(DrawMode::Fill, circle_pos, circle_size);
            surface.square_centered(DrawMode::Fill,      Point2::new(  0.0, -offset), size);
            surface.square_centered(DrawMode::Fill,      Point2::new(  0.0,  offset), size);
            surface.square_centered(DrawMode::Fill,      Point2::new(-offset,   0.0), size);
            surface.square_centered(DrawMode::Fill,      Point2::new( offset,   0.0), size);

            surface.set_color(UIPalette::SelectorFront);
            surface.square_centered(DrawMode::Line(2.0), Point2::new(  0.0, -offset), size);
            surface.square_centered(DrawMode::Line(2.0), Point2::new(  0.0,  offset), size);
            surface.square_centered(DrawMode::Line(2.0), Point2::new(-offset,   0.0), size);
            surface.square_centered(DrawMode::Line(2.0), Point2::new( offset,   0.0), size);

            for (direction, item) in self.player.hands.iter() {
                let direction : Vector2<f32> = direction.into();
                let mut s = PushTransform::new(surface, convert(Translation::from(direction * offset)));
                item.draw(&mut s);
            }

            surface.set_color(UIPalette::SelectorFront);
            surface.circle(DrawMode::Line(2.0), circle_pos, circle_size);
        }
    }
    
    #[derive(Clone, Copy)]
    enum UIPalette {
        SelectorBack,
        SelectorFront,
        ButtonA(bool),
        ButtonB(bool),
        Text,
        StatIndicatorBack,
        StatIndicatorSleep,
        StatIndicatorThirst,
        StatIndicatorHunger,
    }

    impl Color for UIPalette {
        fn into_rgba(self) -> [f32; 4] {
            match self {
                UIPalette::SelectorFront => [0.80, 0.80, 0.80, 1.0],
                UIPalette::SelectorBack => [0.30, 0.30, 0.30, 1.0],
                UIPalette::ButtonA(pressed) => [0.0, if pressed { 1.0 } else { 0.8 }, 0.0, 1.0],
                UIPalette::ButtonB(pressed) => [if pressed { 1.0 } else { 0.8 }, 0.0, 0.0, 1.0],
                UIPalette::Text    => [1.0, 1.0, 1.0, 1.0],
                UIPalette::StatIndicatorBack => [0.7, 0.7, 0.7, 1.0],
                UIPalette::StatIndicatorSleep => [0.9, 0.9, 0.2, 1.0],
                UIPalette::StatIndicatorThirst => [0.3, 0.3, 0.9, 1.0],
                UIPalette::StatIndicatorHunger => [0.9, 0.3, 0.3, 1.0],
            }
        }
    }
    
    pub enum Text {
        ButtonA,
        ButtonB,
        DrinkBottle,
        FillBottle,
        PickUp,
    }
    
    impl Text {
        pub fn str(&self) -> &'static str {
            match self {
                Text::ButtonA => "A",
                Text::ButtonB => "B",
                _ => "...",
            }
        }
        pub fn width(&self) -> f32 {
            match self {
                Text::ButtonA => 10.0,
                Text::ButtonB => 11.0,
                _ => 20.0,
            }
        }
        pub fn center(&self) -> Vector2<f32> {
            Vector2::new(self.width() / 2.0, 8.0)
        }
    }
}

#[derive(Clone, Copy)]
enum Palette {
    Player,
    PlayerSkin,
    PlayerEyes,
    PlayerHair,
    Bag,
    Tent,
    Unknown,
    Void,
    Glass,
    Water,
    Grass,
    TallGrass,
}

impl Color for Palette {
    fn into_rgba(self) -> [f32; 4] {
        match self {
            Palette::Player => [1.00, 1.00, 1.00, 1.0],
            Palette::PlayerSkin => [1.00, 0.80, 0.70, 1.0],
            Palette::PlayerEyes => [0.10, 0.20, 0.90, 1.0],
            Palette::PlayerHair => [0.80, 0.30, 0.20, 1.0],
            Palette::Tent => [0.10, 1.00, 0.10, 1.0],
            Palette::Bag => [0.40, 0.26, 0.13, 1.0],
            Palette::Unknown => [1.00, 1.00, 0.00, 1.0],
            Palette::Void => [0.00, 0.00, 0.00, 1.0],
            Palette::Water => [0.30, 0.50, 0.90, 1.0],
            Palette::Glass => [0.90, 0.90, 0.90, 0.8],
            Palette::Grass => [0.20, 0.70, 0.20, 0.3],
            Palette::TallGrass => [0.20, 0.50, 0.20, 1.0],
        }
    }
}

impl<S> Draw<S> for World
where
    S: DrawPrimitives,
{
    fn draw(&self, surface: &mut S) {
        let mut ids: Vec<NodeIndex> = self.content.node_indices().collect();

        ids.sort_by(|a, b| {
            let a = self.content[a.clone()].position.y;
            let b = self.content[b.clone()].position.y;
            a.partial_cmp(&b).unwrap()
        });

        for id in ids {
            self.content[id].draw(surface);
        }
    }
}

impl World {
    fn new() -> Self {
        let mut content = StableGraph::new();

        let player_id = content.add_node(Entity::new(EntityKind::Player(Player::new()), Point2::new(150.0, 100.0)));
        content.add_node(Entity::new(EntityKind::Pond(40.0), Point2::new(200.0, 130.0)));
        content.add_node(Entity::new(EntityKind::Pond(60.0), Point2::new(280.0, 120.0)));
        content.add_node(Entity::new(EntityKind::Pond(50.0), Point2::new(240.0, 180.0)));
        // content.add_node(Entity::new(EntityKind::Monster(Monster::Mouse), Point2::new(50.0, 80.0)));
        // content.add_node(Entity::Tree(Tree::new(Point2::new(100.0, 200.0))));
        // content.add_node(Entity::Tree(Tree::new(Point2::new(200.0, 180.0))));
        // content.add_node(Entity::Monster(Monster::new(Point2::new(200.0, 200.0))));
        // let player_id = content.add_node(Entity::Player(Player::new()));

        Self { player_id, content, speed: 100.0, delta: 0.0, spawn_cooldown: 3.0 }
    }
    
    pub fn water_in_front_of_player(&self) -> bool {
        let pos = self.get_player_position();
        let player = self.get_player().clone();
        let distance = 20.0;
        let water_pos = pos + player.log_speed.normalize() * distance;

        let mut ids: Vec<NodeIndex> = self.content.node_indices().collect();
        for id in ids {
            let e = &self.content[id];
            if let EntityKind::Pond(size) = e.kind.clone() {
                let d = (water_pos - e.position).norm();
                if d < size {
                    return true
                }
            }
        }
        return false
    }

    pub fn get_player(&self) -> &Player {
        let pe = &self.content[self.player_id];
        if let EntityKind::Player(ref player) = pe.kind {
            player
        } else {
            panic!("Not a player!")
        }
    }
    pub fn get_player_mut(&mut self) -> &mut Player {
        let pe = &mut self.content[self.player_id];
        if let EntityKind::Player(ref mut player) = pe.kind {
            player
        } else {
            panic!("Not a player!")
        }
    }
    pub fn get_player_position(&self) -> Point2<f32> {
        let pe = &self.content[self.player_id];
        pe.position.clone()
    }
}

impl<B> Update<B> for World
where
    B: ElapsedDelta + JoystickProvider,
{
    fn update(&mut self, backend: &mut B) {

        self.delta = backend.delta();
        let mut is_alive = true;

        let ids: Vec<NodeIndex> = self.content.node_indices().collect();
        for id in ids {
            let mut o = self.content[id].clone();
            o.update(self);
            self.content[id] = o;
        }
    }
}

impl Update<World> for Entity {
    fn update(&mut self, w: &mut World) {
        let self_size = self.kind.size();
        match self.kind {
            EntityKind::Player(ref mut player) => {
                let mut s = player.log_speed.norm().log2();
                if s.abs() > 0.02 {
                    let movement = player.log_speed.normalize() * s;
                    let eids: Vec<NodeIndex> = w.content.node_indices().collect();
                    for eid in eids {
                        let other = w.content[eid].clone();
                        if let EntityKind::Player(_) = other.kind {
                            continue;
                        }
                        let d = ((self.position + movement) - other.position).norm();
                        s = (d - (self_size + other.kind.size())).max(0.0).min(s);
                    }
                    let movement = player.log_speed.normalize() * s;
                    self.position = self.position + movement;
                }
                
                player.sleep = (player.sleep - w.delta() / 60.0).max(0.0);
                player.hunger = (player.hunger - w.delta() / 30.0).max(0.0);
                player.thirst = (player.thirst - w.delta() / 15.0).max(0.0);
            },
            _ => {
                
            }
        }
    }
}


use mursten::Scene;

struct Walden {
    world: World,
    button_a: Button,
    button_b: Button,
    selector: Selector,
    indicators: Vec<StatIndicator>,
    camera_pos: Vector2<f32>,
}

impl Walden {
    fn new() -> Self {
        let world = World::new();
        let player = world.get_player().clone();
        Walden {
            world: world,
            button_a: Button::a(),
            button_b: Button::b(),
            selector: Selector::new(player.clone()),
            camera_pos: Vector2::new(0.0, 0.0),
            indicators: vec![
                StatIndicator::Hunger(player.clone()),
                StatIndicator::Thirst(player.clone()),
                StatIndicator::Sleep(player.clone()),
            ],
        }
    }
    fn trigger_action(&mut self) {
        let player = self.world.get_player().clone();
        
        if let Some(item) = player.hands.get(&player.current_hand) {
            eprintln!("Doing action with {:?}", item);
            if let Some(resolved_item) = item.clone().do_action(&mut self.world) {
                self.world.get_player_mut().hands.insert(player.current_hand, resolved_item);
            }
            else {
                self.world.get_player_mut().hands.remove(&player.current_hand);
            }
        }
        else {
            eprintln!("Doing action with empty hand");
        }
    }
    fn swap_item(&mut self, dpad: Dpad) {
        let player = self.world.get_player().clone();
        let item = player.hands.get(&player.current_hand);
        let new_item = player.hands.get(&dpad);
        eprintln!("Swapping {:?} for {:?}", item, new_item);
        
        self.world.get_player_mut().current_hand = dpad;
    }
}

impl<S> Draw<S> for Walden
where
    S: DrawPrimitives,
{
    fn draw(&self, surface: &mut S) {
        surface.clear(Palette::Grass);

        use mursten::graphics::PushTransform;
        {
            self.world.draw(
                &mut PushTransform::new(
                    surface,
                    convert(Translation2::from(self.camera_pos))
                )
            );
        }

        
        let player = self.world.get_player().clone();
         
        
        if let Some(item) = player.hands.get(&player.current_hand) {
            item.draw(
                &mut PushTransform::new(
                    surface,
                    convert(Translation2::from(Vector2::new(260.0, 220.0)))
                )
            );
        }
        self.button_a.draw(&mut PushTransform::new(surface, convert(Translation2::from(Vector2::new(280.0, 220.0)))));
        self.button_b.draw(&mut PushTransform::new(surface, convert(Translation2::from(Vector2::new(300.0, 200.0)))));
        if self.selector.is_visible() {
            self.selector.draw(&mut PushTransform::new(surface, convert(Translation2::from(Vector2::new(160.0, 120.0)))));
        }
        
        for (i, indicator) in self.indicators.iter().enumerate() {
            indicator.draw(
                &mut PushTransform::new(
                    surface,
                    convert(Translation2::from(Vector2::new(25.0 + 40.0 * i as f32, 25.0)))
                )
            );
        }
        
        surface.present();
    }
}

impl<C> Update<C> for Walden
where
    C: ElapsedDelta + JoystickProvider, 
{
    fn update(&mut self, context: &mut C) {
        if let Some(jid) = context.available_joysticks().first() {
            let joystick = context.joystick(*jid);
            {
                {
                    let player = self.world.get_player_mut();
                    player.t += context.delta();
                    let direction = player.log_speed.normalize();

                    let new_speed = if self.selector.state == SelectorState::Idle {
                        match joystick.d_pad.clone() {
                            Some(direction) => {
                                let d : Vector2<_> = direction.into();
                                d * 2.0
                            },
                            None => direction,
                        }
                    }
                    else { direction };
                    
                    let d = new_speed - player.log_speed;
                    player.log_speed += d / 5.0;
                    
                    if player.log_speed.norm() < 1.0 {
                        player.log_speed = player.log_speed.normalize();
                    }
                    
                    self.selector.player = player.clone();
                    for ref mut indicator in self.indicators.iter_mut() {
                        indicator.set_player(player);
                    }
                }

                self.button_a.pressed = joystick.a.is_pressed();
                self.button_b.pressed = joystick.b.is_pressed();


                let state = self.selector.state.clone();
                let next_state = match state {
                    SelectorState::Idle => {
                        if joystick.a.is_pressed() {
                            SelectorState::Deciding(0.5)
                        }
                        else {
                            SelectorState::Idle
                        }
                    },
                    SelectorState::Deciding(timeout) => {
                        self.selector.choice = joystick.d_pad;
                        
                        if joystick.a.is_not_pressed() {
                            self.trigger_action();
                            SelectorState::Idle
                        }
                        else if joystick.d_pad.is_some() {
                            SelectorState::ItemChosed
                        } 
                        else if timeout <= 0.0 {
                            SelectorState::AboutToCancel
                        }
                        else {
                            SelectorState::Deciding(timeout - context.delta())
                        }
                    },
                    SelectorState::ItemChosed => {
                        self.selector.choice = joystick.d_pad;
                        
                        if joystick.d_pad.is_some() {
                            if joystick.a.is_not_pressed() {
                                self.swap_item(joystick.d_pad.unwrap());
                                SelectorState::Idle
                            }
                            else {
                                SelectorState::ItemChosed
                            }
                        }
                        else {
                            SelectorState::AboutToCancel
                        }
                    },
                    SelectorState::AboutToCancel => {
                        self.selector.choice = joystick.d_pad;
                        
                        if joystick.d_pad.is_some() {
                            if joystick.a.is_pressed() {
                                SelectorState::ItemChosed
                            }
                            else {
                                SelectorState::AboutToCancel
                            }
                        }
                        else {
                            if joystick.a.is_not_pressed() {
                                SelectorState::Idle
                            }
                            else {
                                SelectorState::AboutToCancel
                            }
                        }
                    }
                };
                self.selector.state = next_state;
                self.selector.update(context);
            }
        }
        
        {
            let pos = self.world.get_player_position() - Vector2::new(160.0, 120.0);
            let offset = self.world.get_player().log_speed.normalize() * 40.0;
            let offset = (pos + offset).coords * -1.0;
            self.camera_pos = self.camera_pos * 0.9 + offset * 0.1;
        }
        
        self.world.update(context);
    }
}

impl Scene for Walden {}

use mursten::Game;
use mursten_ggez_backend::GgezBackend;

fn main() {
    Game::new(GgezBackend::new(320, 240))
        .run(Walden::new());
}

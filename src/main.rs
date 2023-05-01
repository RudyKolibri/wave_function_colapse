use macroquad::{prelude::*, rand::{srand,gen_range}};
use ::rand::prelude::*;
use ::rand::distributions::WeightedIndex;

#[derive(Debug, Clone)]
struct Tile {
    tile_types : Vec<(char, i32)>,
    colapsed : bool,
    position : [i32; 2],
    index : i32 //makes searching an index of a tile a lot faster
}
const TILES_PER_ROW: i32 = 20;
#[macroquad::main("Wave_function_colapse")]

async fn main () {
    let mut deep_water = gen_range(1, 5);
    let mut water = gen_range(1, 5);
    let mut sand = gen_range(1, 5);
    let mut grass = gen_range(1, 5);
    let mut woods = gen_range(1, 5);

    let mut tile_list = run(deep_water,water,sand,grass,woods); //first run, we want the user to see instandly what they can expact
    next_frame().await; //we call next frame, that the window updates
    loop {
        if is_key_pressed(KeyCode::Space) { //making that if you press space that it generates a new one
            tile_list.clear();
            deep_water = gen_range(1, 5);
            water = gen_range(1, 5);
            sand = gen_range(1, 5);
            grass = gen_range(1, 5);
            woods = gen_range(1, 5);
            tile_list = run(deep_water,water,sand,grass,woods);

        }
        if tile_list.len() >= 1 {
            draw_tiles(tile_list.clone());
            next_frame().await
        } else { //if it breaks it would return an empty tile_list vector, so if its less then 1 it means it broke, because of that we generate a new one
            deep_water = gen_range(1, 5);
            water = gen_range(1, 5);
            sand = gen_range(1, 5);
            grass = gen_range(1, 5);
            woods = gen_range(1, 5);
            tile_list = run(deep_water,water,sand,grass,woods)
        }
    }
}
fn run (deep_water : i32, water : i32, sand : i32, grass : i32, woods : i32) -> Vec<Tile>{
    let mut rng = thread_rng();
    let time_start = get_time();
    let mut tile_list : Vec<Tile> = Vec::new();

    srand((get_time() * 100000.0) as u64); //random seed, this way we dont get evrytime the same map

    let mut new_tile : Tile;
    let mut index_tile = 0;
    //code to generate all tiles, set there position and option + index
    for y in 0..TILES_PER_ROW {
        for x in 0..TILES_PER_ROW {
            new_tile = Tile {
                tile_types : [('1',deep_water),('2',water),('3',sand),('4',grass),('5',woods)].to_vec(),
                colapsed : false,
                position : [x, y],
                index : index_tile
            };
            tile_list.push(new_tile);  //we add the tile to the vector
            index_tile += 1 //we make the tile index goes + 1 this way we dont have 2 tiles with the same indew
        }
    }


    let mut num_collapsed_tiles = 0; //this is the count where we store how mutch tiles we have colapsed

    while num_collapsed_tiles != TILES_PER_ROW * TILES_PER_ROW { //while we dont have all colapsed
        let mut tilenotc_list : Vec<Tile> = Vec::new(); //the vector where we store all NOT colapsed tiles, this way we run on a smaller list
        //gets all tiles that are not jet collapsed
        for tile in &tile_list {
            if tile.colapsed == false {
                tilenotc_list.push(tile.clone())
            }
        }
        //this is or template to check if there are smaller tiles
        let mut tile_lowest_entropy = Tile {
            tile_types : [('1',1),('2',1),('3',1),('4',1),('5',1)].to_vec(),
            colapsed : false,
            position : [-1, 0],
            index : -1
        };
        //we search the one with the lowest entropy (options)
        for i in &tilenotc_list {
            if i.tile_types.len() <= tile_lowest_entropy.tile_types.len() {
                tile_lowest_entropy = i.clone();
            }
        }
        //we get an random option and colapse the tile
        
        let items = tile_lowest_entropy.clone().tile_types;
        let dist2 = WeightedIndex::new(items.iter().map(|item| item.1)).unwrap();
        tile_lowest_entropy.tile_types.clear();
        tile_lowest_entropy.tile_types.push((items[dist2.sample(&mut rng)].0 as char,1));
        tile_lowest_entropy.colapsed = true;
        num_collapsed_tiles += 1;

        let index = tile_lowest_entropy.index as usize;

        //we add him back to tile_list
        tile_list[index] = tile_lowest_entropy; 

        //we remove options from the neightbers
        let mut neightbers : Vec<Tile> = Vec::new();

        if (index as i32 - TILES_PER_ROW) >= 0 {
            let test_tile = tile_list[index - TILES_PER_ROW as usize].clone();
            if tile_list[index].position[0] == test_tile.position[0] {
                neightbers.push(test_tile)
            }
        }

        if index % TILES_PER_ROW as usize != 0{
            let test_tile = tile_list[index - 1].clone();
            if tile_list[index].position[1] == test_tile.position[1] {
                neightbers.push(test_tile)
            }
        }
        if index + 1 < tile_list.len() && index % TILES_PER_ROW as usize != 1{
            let test_tile = tile_list[index + 1].clone();
            if tile_list[index].position[1] == test_tile.position[1] {
                neightbers.push(test_tile)
            }
        }

        if (index + TILES_PER_ROW as usize) < tile_list.len() {
            let test_tile = tile_list[index + TILES_PER_ROW as usize].clone();
            if tile_list[index].position[0] == test_tile.position[0] {
                neightbers.push(test_tile)
            }
        }



        let origenal = tile_list[index].clone();
        let mut retain: Vec<char> = Vec::new(); //the options we have to remove from the neightbers will be stored in here

        //1 = deep water
        //2 = water
        //3 = sand
        //4 = grass
        //5 = woods

        //the hard coded remover
        if origenal.tile_types.get(0).unwrap().0 == '1' { //tiles_list.get(0).unwrap().0
            retain.push('5');
            retain.push('4');
            retain.push('3')
        }
        if origenal.tile_types.get(0).unwrap().0 == '2' {
            retain.push('5');
            retain.push('4');
        }
        if origenal.tile_types.get(0).unwrap().0 == '3' {
            retain.push('1');
            retain.push('5');
        }
        if origenal.tile_types.get(0).unwrap().0 == '4' {
            retain.push('1');
            retain.push('2');
        };
        if origenal.tile_types.get(0).unwrap().0 == '5' {
            retain.push('1');
            retain.push('2');
            retain.push('3');
        };
        //the we go over the neightbers and remove those options
        for i in &neightbers {
            let mut this_tile = i.clone();
            let mut this_tile_types = this_tile.tile_types.clone();

            for r in retain.clone() {
                this_tile_types.retain(|&x| x.0 != r); //r.to_digit(10).unwrap() as i32
                
            };
            this_tile.tile_types = this_tile_types;

            let index3 = this_tile.index as usize;
            
            tile_list[index3] = this_tile; //we add this tile back to the main list
        };
        //we check if nothing broke if it broke we return an empty vector, else we continue
        for tile in &tile_list{
            if tile.tile_types.len() < 1 {
                tile_list.clear();
                return tile_list;
            }
        }
    }
    println!("time : {:?}", get_time() - time_start);
    return tile_list;
}

fn draw_tiles(tile_list : Vec<Tile>) {
    for tile in tile_list{
        let mut color = BLACK;
        if tile.tile_types.len() == 1 {
            if tile.tile_types.get(0).unwrap().0 == '2' { //origenal.tile_types.get(0).unwrap().0
                color = BLUE
            }
            if tile.tile_types.get(0).unwrap().0 == '3' {
                color = YELLOW
            }
            if tile.tile_types.get(0).unwrap().0 == '4' {
                color = LIME
            }
            if tile.tile_types.get(0).unwrap().0== '5' {
                color = DARKGREEN
            }
            if tile.tile_types.get(0).unwrap().0== '1' {
                color = DARKBLUE
            }
        }
        let x = tile.position[0];
        let y = tile.position[1];
        draw_rectangle(x as f32 * 10.0, y as f32 * 10.0 , 10.0, 10.0, color)
    }
}
use std::{fs, io::BufRead};

use thiserror::Error;

use bevy::{prelude::*, utils::HashMap};

use crate::ants::{board::Board, cell::Cell, item::Item};

const colors: [Color; 16] = [
    Color::ALICE_BLUE,
    Color::AQUAMARINE,
    Color::BLUE,
    Color::ANTIQUE_WHITE,
    Color::RED,
    Color::CRIMSON,
    Color::CYAN,
    Color::GRAY,
    Color::GOLD,
    Color::AZURE,
    Color::BISQUE,
    Color::OLIVE,
    Color::SEA_GREEN,
    Color::MIDNIGHT_BLUE,
    Color::TURQUOISE,
    Color::ORANGE_RED
];

pub type DatasetData = HashMap<String, Vec<[f32; 2]>>;

pub struct DataPlugin {
    pub data_path: String,
}

pub struct DataSetParam {
    pub dataset_path: String,
}

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DataSetParam {
            dataset_path: self.data_path.clone(),
        })
        .add_startup_system(set_data_items);
    }
}

pub fn set_data_items(
    mut commands: Commands,
    dataset_params: Res<DataSetParam>,
    board: Res<Board>,
    mut query: Query<&mut Cell>,
) {
    let dataset = read_and_parse(&dataset_params.dataset_path).unwrap();

    let item_count: usize = dataset.iter().map(|(_, v)| v.len()).sum();
    if (board.size * board.size) < (item_count as i32) {
        panic!("There are more food then cells in the board.");
    }

    for data in dataset {
        let label = data.0;

        for item in data.1 {
            let (x, y) = board.rand_position();

            let mut cell = query.get_mut(board.get_cell_entity(x, y)).unwrap();

            let i: usize = label.parse::<i32>().unwrap().try_into().unwrap();
            let color = colors[i];

            if cell.item.is_none() {
                let entity = commands
                    .spawn()
                    .insert(Item {
                        label: label.clone(),
                        color: color,
                        data: vec![item[0] as f64, item[1] as f64],
                    })
                    .id();

                cell.item = Some(entity);
            }
        }
    }
}

pub fn read_and_parse(dataset_path: &String) -> Result<DatasetData, DatasetError> {
    println!("{}", dataset_path);

    let text = fs::read_to_string(dataset_path).expect("Error while reading dataset");

    let mut data = HashMap::new();
    for (idx, line) in text.lines().enumerate() {
        let idx = idx + 1;
        let line = line.trim();
        if line.is_empty() || line.chars().next().unwrap() == '#' {
            continue;
        }

        let mut itr = line.split_whitespace();
        let x: f32 = itr
            .next()
            .ok_or(DatasetError::MissingArgumentError {
                idx,
                text: line.to_string(),
            })?
            .replace(',', ".")
            .parse()?;

        let y: f32 = itr
            .next()
            .ok_or(DatasetError::MissingArgumentError {
                idx,
                text: line.to_string(),
            })?
            .replace(',', ".")
            .parse()?;

        let g = itr
            .next()
            .ok_or(DatasetError::MissingArgumentError {
                idx,
                text: line.to_string(),
            })?
            .to_string();

        if itr.next().is_some() {
            Err(DatasetError::TooManyArgumentsError {
                idx,
                text: line.to_string(),
            })?
        }

        if !data.contains_key(&g) {
            data.insert(g.clone(), Vec::new());
        }
        data.get_mut(&g).unwrap().push([x, y]);
    }
    Ok(data)
}

#[derive(Debug, Error)]
pub enum DatasetError {
    #[error("Missing argument on line {idx}: {text:?}")]
    MissingArgumentError { idx: usize, text: String },
    #[error("Invalid File Format")]
    InvalidFormatError {
        #[from]
        source: std::str::Utf8Error,
    },
    #[error("Could not parse float")]
    ParsingError {
        #[from]
        source: std::num::ParseFloatError,
    },
    #[error("Too many arguments on line {idx}: {text:?}")]
    TooManyArgumentsError { idx: usize, text: String },
}

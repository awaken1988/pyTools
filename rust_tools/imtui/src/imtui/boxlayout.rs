use std::io::{stdout, Write};
use crossterm::{cursor, terminal, execute, style, event, event::{Event, read, KeyCode}, ExecutableCommand, QueueableCommand};
use crate::imtui::def::{*};

const BORDER_SPACE_X: usize = 1;
const BORDER_SPACE_Y: usize = 1;

pub struct BoxLayout<'a> {
    table: Vec<Vec<Option<BoxLayoutItem<'a>>>>,

    used_max_x: Vec<usize>,
    used_max_y: Vec<usize>,

    items_x: usize,
    items_y: usize,

    expand_x: Vec<usize>,
    expand_y: Vec<usize>,
}

#[derive(Clone)]
struct BoxLayoutItem<'a> {
    widget: &'a Box<dyn Widget>,
    position: Size2D,
    used_size: Size2D,
    offset: Size2D,
}

impl<'a> BoxLayout<'a> {
    pub fn new() -> BoxLayout<'a> {
        return BoxLayout {
            table: Vec::new(),
            used_max_x: Vec::new(),
            used_max_y: Vec::new(),
            items_x: 0,
            items_y: 0,
            expand_x: Vec::new(),
            expand_y: Vec::new(),
        };
    }

    pub fn add(&mut self, widget: &'a Box<dyn Widget>, position: Size2D) {
        self.resize_table(position);

        if self.table[0].len() <= (position.y) {
            let y_diff = (position.y+1)-self.table[0].len();
            for iRow in &mut self.table {
                iRow.append( &mut vec![None; y_diff] );
            }
        }

        for iRow in &self.table {
            println!("len {}", iRow.len());
        }
        println!("___");

        let mut entry = &mut self.table[position.x][position.y];

        if let Some(x) = entry {
            panic!("use position twice");
        }

        *entry = Some(BoxLayoutItem {
            widget: widget,
            position: position,
            used_size: Size2D{x: 0, y: 0},
            offset: Size2D{x: 0, y: 0}, });
    }

    pub fn set_expand_x(&mut self, col: usize, expand: usize) {
        self.expand_x[col] = expand;
    }

    pub fn set_expand_y(&mut self, row: usize, expand: usize) {
        self.expand_y[row] = expand;
    }

    pub fn draw(&mut self) {
        self.calc_layout();

        for iRow in 0..self.table.len() {
            for iCol in 0..self.table[iRow].len() {
                if let Some(col) = &mut self.table[iRow][iCol] {
                    
                    //draw content
                    {
                        let mut content_size = col.used_size;
                        content_size.x -= BORDER_SPACE_X;
                        content_size.x -= BORDER_SPACE_Y;
                        col.widget.draw(col.offset, col.used_size);
                    }
                }

            }
         }

        let mut curr_row_offset = 0;
        for iRow in 0..self.table.len() {
            let mut curr_col_offset = 0;
            for iCol in 0..self.table[iRow].len() {
                
                //draw vertical bordder
                {
                    let offset_x = curr_col_offset + self.used_max_x[iCol] - 1;
                    let start_y  = curr_row_offset;

                    for iRow in 0..(self.used_max_y[iRow]) {
                        stdout().queue( cursor::MoveTo(offset_x as u16, (start_y + iRow) as u16) );
                        stdout().queue( style::Print("|".to_string()) );
                    }
                }

                //draw horizontal border
                {
                    let offset_y = curr_row_offset + self.used_max_y[iRow] - 1;
                    let start_x  = curr_col_offset;

                    for iCol in 0..(self.used_max_x[iCol]) {
                        stdout().queue( cursor::MoveTo((start_x + iCol) as u16, offset_y as u16) );
                        stdout().queue( style::Print("-".to_string()) );
                    }
                }
                
                curr_col_offset += self.used_max_x[iCol];
            }
            curr_row_offset += self.used_max_y[iRow];
        }

        stdout().flush();
    }

    fn resize_table(&mut self, position: Size2D) {
        if self.items_x <= position.x {
            self.items_x = position.x+1;
        }
        if self.items_y <= position.y {
            self.items_y = position.y+1;
        }

        //resize x
        if self.table.len() < self.items_x {
            let diff = self.items_x - self.table.len();
            self.table.append( &mut vec![vec![None; 0]; diff] );     
            self.expand_x.append( &mut vec![0; diff]);

        }

        //resize y
        for iRow in &mut self.table {
            if iRow.len() < self.items_y {
                let y_diff = self.items_y - iRow.len();
                iRow.append( &mut vec![None; y_diff] );
            }
        }

        if self.expand_y.len() < self.items_y {
            let y_diff = self.items_y - self.expand_y.len();
            self.expand_y.append( &mut vec![0; y_diff]);
        }        
    }

    fn calc_layout(&mut self) {

        self.used_max_x = vec![0; self.table.len()];
        self.used_max_y = vec![0; self.table[0].len()];

        let term_size = terminal::size().unwrap();

        //set all widget to min size; all hor/vert cells should have the same size
        for iRow in 0..self.table.len() {
            for iCol in 0..self.table[iRow].len() {
                if let Some(col) = &mut self.table[iRow][iCol] {
                    col.used_size = col.widget.min_space();
                    col.used_size.x += BORDER_SPACE_X;
                    col.used_size.y += BORDER_SPACE_Y;

                    if col.used_size.x > self.used_max_x[iCol] {
                        self.used_max_x[iCol] = col.used_size.x;
                    }
                    if col.used_size.y > self.used_max_y[iRow] {
                        self.used_max_y[iRow] = col.used_size.y;
                    }
                }
            }
        }

        //calculate the offset
        for iRow in 0..self.table.len() {
            for iCol in 0..self.table[iRow].len() {
                if let Some(col) = &mut self.table[iRow][iCol] {
                    let mut offset = Size2D{x: 0, y: 0};

                    for i in 0..iCol {
                        offset.x += self.used_max_x[i];
                    }
                    for i in 0..iRow {
                        offset.y += self.used_max_y[i];
                    }

                    col.offset = offset;
                }
            }
        }   

        //expand x
        if false
        {
            let used: usize = self.used_max_x.iter().sum();
            let rest = (term_size.0 as usize) - used;
            let expand_sum: usize = self.expand_x.iter().sum();

            let mut added_space = 0;

            if expand_sum > 0 {
                for i_x in 0..self.items_x {
                    let curr_extra_space = (self.expand_x[i_x] * rest) / expand_sum;
                    for i_y in 0..self.items_y {
                        if let Some(cell) = &mut self.table[i_x][i_y] {
                            cell.used_size.x += curr_extra_space;
                            cell.offset.x += added_space;
                        }
                    }
                    added_space += curr_extra_space;
                    self.used_max_x[i_x] += added_space;
                }
            }
        }

        //expand y
        if false
        {
            let used: usize = self.used_max_y.iter().sum();
            let rest = (term_size.1 as usize) - used;
            let expand_sum: usize = self.expand_y.iter().sum();

            let mut added_space = 0;

            if expand_sum > 0 {
                for i_x in 0..self.items_y {
                    let curr_extra_space = (self.expand_x[i_x] * rest) / expand_sum;
                    for i_y in 0..self.items_x {
                        if let Some(cell) = &mut self.table[i_x][i_y] {
                            cell.used_size.y += curr_extra_space;
                            cell.offset.y += added_space;
                        }
                    }
                    added_space += curr_extra_space;
                    self.used_max_y[i_x] += added_space;
                }
            }
        }
    }
}
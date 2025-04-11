use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use csv;

#[derive(Debug, Clone)]
enum ColumnVal {
    One(String),
    Two(bool),
    Three(f64),
    Four(i64),
}

impl ColumnVal {
    fn unwrap1(&self) -> Result<String, Box<dyn Error>> {
        match self {
            ColumnVal::One(s) => Ok(s.clone()),
            _ => Err(Box::new(MyError(format!(
                "{:?} is not a String",
                self
            )))),
        }
    }

    fn unwrap2(&self) -> Result<bool, Box<dyn Error>> {
        match self {
            ColumnVal::Two(b) => Ok(*b),
            _ => Err(Box::new(MyError(format!(
                "{:?} is not a bool",
                self
            )))),
        }
    }

    fn unwrap3(&self) -> Result<f64, Box<dyn Error>> {
        match self {
            ColumnVal::Three(f) => Ok(*f),
            _ => Err(Box::new(MyError(format!(
                "{:?} is not a float",
                self
            )))),
        }
    }

    fn unwrap4(&self) -> Result<i64, Box<dyn Error>> {
        match self {
            ColumnVal::Four(i) => Ok(*i),
            _ => Err(Box::new(MyError(format!(
                "{:?} is not a Int",
                self
            )))),
        }
    }

}

#[derive(Debug, Clone)]
struct ColumnInformation {
    types: u32,
    data: Vec<ColumnVal>,
}

#[derive(Debug)]
struct DataFrame {
    columns: HashMap<String, ColumnInformation>,
    order: Vec<String>,
    numRows: u32,
}

impl Clone for DataFrame {
    fn clone(&self) -> Self {
        DataFrame {
            columns: self.columns.clone(),
            order: self.order.clone(),
            numRows: self.numRows,
        }
    }
}

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

fn infer_type(o: &str) -> u32 {

    // Check boolean
    if o == "false" || o == "true" {
        return 2;
    }

    // Check int
    if o.parse::<i64>().is_ok() {
        return 4;
    }

    // Check float
    if o.parse::<f64>().is_ok() {
        return 3;
    }

    // Else String
    1
}

impl DataFrame {
    fn new() -> Self {
        DataFrame {
            columns: HashMap::new(),
            order: Vec::new(),
            numRows: 0,
        }
    }

    fn read_csv(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_path(path)?;

        let mut first_row = true;

        for result in rdr.records() {
            let r = result?;

            if first_row {
                for elem in r.iter() {
                    self.columns.insert(
                        elem.to_string(),
                        ColumnInformation {
                            types: 0,
                            data: Vec::new(),
                        },
                    );
                    self.order.push(elem.to_string());
                }
                first_row = false;
                continue;
            }

            self.numRows += 1;

            for (column_name, elem) in self.order.iter().zip(r.iter()) {
                let target_column = self.columns.get_mut(column_name).unwrap();
                let data = &mut target_column.data;
                let t = &mut target_column.types;

                let i_type: u32 = infer_type(elem);

                if *t == 0 {
                    *t = i_type;
                } else if *t != i_type {
                    return Err(Box::new(MyError(format!(
                        "Column {} was type {} but found type {}",
                        column_name, t, i_type
                    ))));
                }

                match i_type {
                    1 => data.push(ColumnVal::One(elem.to_string())),
                    2 => data.push(ColumnVal::Two(elem.parse::<bool>()?)),
                    3 => data.push(ColumnVal::Three(elem.parse::<f64>()?)),
                    4 => data.push(ColumnVal::Four(elem.parse::<i64>()?)),
                    _ => return Err(Box::new(MyError("Unknown type".to_string()))),
                }
            }
        }

        Ok(())
    }

    fn print(&self) {
        for column_name in &self.order {
            print!("{:^20}", column_name);
        }
        println!();

        for i in 0..self.numRows as usize {
            for column_name in &self.order {
                let val = &self.columns[column_name].data[i];
                match val {
                    ColumnVal::One(s) => print!("{:^20}", s),
                    ColumnVal::Two(b) => print!("{:^20}", b),
                    ColumnVal::Three(f) => print!("{:^20}", f),
                    ColumnVal::Four(n) => print!("{:^20}", n),
                }
            }
            println!();
        }
    }

    fn add_column(mut self, column_name: String, data: Vec<ColumnVal>) -> Result<Self, Box<dyn Error>> {
        if data.len() as u32 != self.numRows {
            return Err(Box::new(MyError(format!(
                "Column has {} rows but DF has {}",
                data.len(),
                self.numRows
            ))));
        }

        let t = match &data[0] {
            ColumnVal::One(s) => 1,
            ColumnVal::Two(b) => 2,
            ColumnVal::Three(f) => 3,
            ColumnVal::Four(i) => 4,
        };

        self.columns.insert(column_name.clone(), ColumnInformation { types: t, data });
        self.order.push(column_name);

        Ok(self)
    }

    fn merge_frame(mut self, DF2: Self) -> Result<Self, Box<dyn Error>> {

        // Check columns
        if self.order.len() != DF2.order.len() {

            for o in self.order.iter() {
                if (!DF2.order.contains(o) || self.columns.get(o).unwrap().types != DF2.columns.get(o).unwrap().types) {
                return Err(Box::new(MyError(format!(
                    "DF1 has column {} that is not present or similar in DF2",
                    o
                )))); 
                }
            }

        }

        // Merge

        self.numRows += DF2.numRows;

        for (column_name, values) in &DF2.columns {

            let data = &mut self.columns.get_mut(column_name).unwrap().data;

            for v in values.data.iter() {

                data.push(v.clone());

            }

        }

        Ok(self)

    }

    fn restrict_columns(mut self, selected: Vec<String>) -> Result<Self, Box<dyn Error>> {

        // Check all selected in DF
        for s in selected.iter() {
            if !self.order.contains(s) {
                return Err(Box::new(MyError(format!(
                    "Column {} not in DF",
                    s
                ))));
            }
        }

        for o in self.order.iter() {

            // Purge
            if (!selected.contains(o)) {
                self.columns.remove(o);
            }

        }

        self.order = selected;

        Ok(self)

    }

    fn filter(
        mut self,
        column_name: String,
        operation: fn(&ColumnVal) -> bool,
    ) -> Result<Self, Box<dyn Error>> {
        
        let mut i = 0;

        // I know I am iterating over an array and modifying it which break all laws of good coding but I am tired and lacking in judgement right now
        while true {

            if i >= self.numRows { break };

            let item = &self.columns.get(&column_name).unwrap().data[i as usize];

            let mut goBack: bool = false;

            if !operation(item) {

                self.numRows -= 1;

                for (cn, value) in &mut self.columns {

                    value.data.remove(i as usize);

                }

                goBack = true;

            }

            i += 1;

            if goBack {
                i -= 1;
            }

        }

        Ok(self)

    }

    fn column_op(
        &mut self,
        columns: &[String],
        operation: fn(&Vec<Vec<ColumnVal>>) -> Vec<ColumnVal>,
        row_wise: bool,
    ) -> Vec<ColumnVal> {

        let mut returns: Vec<Vec<ColumnVal>> = vec![];
        
        for column_name in columns.iter() {

            returns.push(
                self.columns.get(column_name).unwrap().data.clone()
            );

        }

        if !row_wise {

            // Returns
            return operation(&returns);

        }

        let mut r: Vec<ColumnVal> = vec![];

        // Iterate over rows
        for i in 0..self.numRows {

            let mut temp: Vec<ColumnVal> = vec![];

            // Iterate over columns
            for col in returns.iter() {
                temp.push(
                    col[i as usize].clone()
                );
            }

            r.push(
                operation(&vec![temp])[0].clone()
            );

        }

        r

    }
}

fn main() {
    let mut df1 = DataFrame::new();
    let mut df2 = DataFrame::new();

    df1.read_csv("data/df1.csv").unwrap();
    df2.read_csv("data/df2.csv").unwrap();

    println!("{:^120}", "Loaded from CSV");

    df1.print();

    println!();
    println!();

    println!("{:^120}", "Testing Clone");

    df1.clone().print();

    println!();
    println!();

    let mut hof = Vec::<ColumnVal>::new();
    for i in 0..5 {
        hof.push(ColumnVal::Two(i % 2 == 0));
    }

    let dfh = df1.clone().add_column("hall_of_fame".to_string(), hof).unwrap();

    println!("{:^120}", "Testing Add Column");

    dfh.print();

    println!();
    println!();

    let mut dfc = df1.clone().merge_frame(df2).unwrap();

    println!("{:^120}", "Testing Merge");

    dfc.print();

    println!();
    println!();

    let mut dfs = dfc.clone().restrict_columns(vec!["Name".to_string(), "PPG".to_string()]).unwrap();

    println!("{:^120}", "Testing Get Columns [\"Name\", \"PPG\"]");

    dfs.print();

    println!();
    println!();

    let mut dff = dfs.clone().filter("PPG".to_string(), | ppg | ppg.unwrap3().unwrap() > 25.0).unwrap();

    println!("{:^120}", "Testing Filter PPG > 25.0");

    dff.print();

    println!();
    println!();

    // Median

    let dfco = df1.clone().column_op(&vec!["PPG".to_string()], |num| {
        
        // unwrap to floats

        let mut fs: Vec<f64> = vec![];

        for f in num[0].iter() {
            fs.push(
                f.unwrap3().unwrap()
            );
        }

        fs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if fs.len() % 2 == 0 {
            // Even

            return vec![ColumnVal::Three(
                (fs[(fs.len() as f64 / 2.0) as usize] +
                fs[((fs.len() as f64 / 2.0) - 1.0) as usize]) / 2.0
            )];
        }

        else {

            return vec![ColumnVal::Three(
                fs[(fs.len() as f64 / 2.0).floor() as usize]
            )]

        }

    }, false);

    println!("Median is {:?}", dfco[0].unwrap3().unwrap()); // as Vec

    println!();
    println!();

    // Subtracting

    let sub_columns = df1.column_op(&vec!["TotalPoints".to_string(), "YearBorn".to_string()], | num | {

        let mut is: Vec<i64> = Vec::new();
    
        for i in num[0].iter() {
            is.push(
                i.unwrap4().unwrap()
            )
        }

        return vec![ColumnVal::Four(is[0] - is[1])];
    }, true);

    println!("Subtracing Total Points {:?}", sub_columns); // as Vec

    println!();
    println!();

}

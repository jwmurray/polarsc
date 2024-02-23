/* 
* This program reads the Iris dataset from a CSV file located at src/data/iris.csv.
It assumes the CSV has headers.

1. Filter:
It filters the rows where the column 'sepal_length' is greater than 5.0.

3. Aggregation:
For each group (species in this case), it calculates the sum of the other columns.

Output:
The resulting DataFrame has a shape of (3,5), indicating that it has 3 rows and 5 columns.
the rows represent the three species of Iris flowers: 'setosa', 'versicolor', and 'virginica'.
The columns represent:
- 'species'
- 'sepal_length'
- 'sepal_width'

For example, for  ....

*/

use std::vec;

// Import necessary modules from the polars crate
use::polars::prelude::*;

// Define the main function that returns a Result type.
// If evertything is OK, it returns `(df)`, otherwise it returns a PolarsError.
pub fn read_csv() ->Result<DataFrame, PolarsError> {
    // Read the CSV file using the polars::prelude::CsvReader::from_path method
    let df = LazyCsvReader::new("src/data/iris.csv")
        .has_header(true)
        .finish()?

        // Filter the rows where the column "sepal_length" is greater than 5.0
        .filter(col("sepal_length").gt(lit(5.0)))

        // Group the filtered DataFrame by the "species" column
        .group_by(vec![col("species")])

        // Aggregate the groups by summing the specified columns for each group
        .agg(&[
            col("sepal_length").sum(),
            col("sepal_width").sum(),
            col("petal_length").sum(),
            col("petal_width").sum(),
        ])

        // Trigger computation adn collect the result into a Dataframe
        .collect()?;

    // Return the DataFrame
    Ok(df)
}
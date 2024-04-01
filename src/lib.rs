

pub mod AAStimeseries{

    use num::complex::Complex;

    pub struct TimeSeries<T>{
        pub metadata : Metadata,
        pub internalsegment : Box<Segment::InternalSegment<T>>,
        pub externalsegment: ExternalSegment,
        pub linkedsegment: LinkedSegment,
    }

    pub struct Metadata{
        pub name: String,
        pub description: String

    }


    pub mod Segment {
        
        use num::complex::Complex;


        pub struct InternalSegment<T>{
            pub name: String,
            pub description: String,
            pub record_count: i64,
            pub start_time: i64,
            pub end_time: i64,
            pub duration: i64,
            pub sampling_interval: String,
            pub state: String,
            pub last_updatet: String,
            pub records: Box<Vec<super::Records::Record<T>>>,
        }

        pub struct ExternalSegment{
            pub name: String,
            pub description: String,
            pub record_count: i64,
            pub start_time: i64,
            pub end_time: i64,
            pub duration: i64,
            pub sampling_interval: String,
            pub state: String,
            pub last_updatet: String,
            pub data: String,
        }

        pub struct LinkedSegment{
            pub name: String,
            pub description: String,
            pub record_count: i64,
            pub start_time: i64,
            pub end_time: i64,
            pub duration: i64,
            pub sampling_interval: String,
            pub state: String,
            pub last_updatet: String,
            pub endpoint: String,
            pub query: String,
        }



    }

    pub mod Records{
        
        use num::complex::Complex;
        use std::f64::consts::PI;
        use std::fs::File;
        use std::io::Write;


        pub struct Record <T>{
            pub time: i64,
            pub series: Vec<T>,
        }
        
        
        impl Record<Complex<f64>>{
            pub fn create_sin_series(&mut self, amp: f64, freq: f64, sampl_rate: f64, number_of_samples: i32){
                let mut tmp_series = Vec::new();
                for t in 0..number_of_samples{
                    let t = t as f64;
                    let value = Complex::new(amp*f64::sin(freq*PI*2.0*t/sampl_rate), 0_f64);
                    tmp_series.push(value);
                }
                self.series = tmp_series;
            }
        
            pub fn create_fourier_transformed(timeseries: &Vec<Complex<f64>>) -> Vec<f64> {//Vec<Complex<f64>> {
                let mut transformed_timeseries = Vec::new();
                let length = timeseries.len();
                let length_f64 = timeseries.len() as f64;
                for element in 0..length{
                    let element = element as f64;
                    let mut transformed_number = Complex::new(0_f64, 0_f64);
                    for (ite, number) in timeseries.iter().enumerate() {
                        let ite = ite as f64;
                        transformed_number = number*Complex::new(-f64::cos(2.0*PI*element*ite/length_f64), -f64::sin(2.0*PI*element*ite/length_f64)) + transformed_number
                    }
                    //transformed_timeseries.push(transformed_number);
                    let amplitude = f64::sqrt(transformed_number.re*transformed_number.re/length_f64+transformed_number.im*transformed_number.im/length_f64);
                    transformed_timeseries.push(amplitude);
                }
                transformed_timeseries
            }
        
            pub fn create_export(frequence_spectrum: &Vec<f64>, file_name: &str){
                let mut file_result = File::create(file_name);
            
                let mut file = match file_result {
                    Ok(file) => file,
                    Err(error) => panic!("Problem opening the file: {:?}", error),
                };
            
                for value in frequence_spectrum {
                    write!(file, "{}\n", value);
                  }
            }
        }

    }
}




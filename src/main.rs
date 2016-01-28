const MAX: usize = 5000;

fn main() {
    //First generate the first 25 ludic numbers
    
    print!("1,");
    let mut array: [i32; MAX] = [0; MAX];

    //Construct the first array
    let mut num_inicial:i32 = 2;
    for i in 0..array.len()
    {
    	array[i] = num_inicial;
    	num_inicial += 1;
    }
    //print_array(&array);

    /*for i in 0..array.len()
    {
    	print!(",{}",array[i]);
    }
    */

    
    for _ in 0..25
    {
    	let next_ludic = array[0];
    	print!("{},", next_ludic);
    	let mut contador = 0;    	
	    for i in 0..array.len()
	    {
	    	//modulo operation:
	    	//((a % b) + b) % b
	    	let modulo = ((i as i32 % next_ludic) + next_ludic) % next_ludic;
	    	//print!("{},",modulo);
	    	if modulo != 0
	    	{
	    		//let temp_val = ; 
	    		array[contador] = array[i];
	    		//print!("{},", temp_val);
	    		contador += 1;
	    	}
	    }
	    //println!("");
	    //print_array(&array);

	}

	
}

fn print_array(x: &[i32; MAX])
{
	for i in 0..x.len()
	{
		print!("{},", x[i]);
	}
	println!("");
}

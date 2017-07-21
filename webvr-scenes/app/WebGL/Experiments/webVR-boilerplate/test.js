
function SelectionSort(){
 var  arr = [7, 4, 3, 5, 8, 6, 20, 12] ;
    //  alert(arr);

    var temp ;
    var minIndex ;

    for (var i = 0 ; i < arr.length ; i++){

	minIndex = i ;
	for(var j = i ; j < arr.length ; j++){

	    if(arr[j] < arr[minIndex]){
		minindex = j ;

	    } // end of if-statement

	}// end of inner for loop
	temp = arr[minIndex];
	arr[minIndex] = arr[j] ;
	arr[j] = temp ;


    } // end of outer for loop
    alert(arr);


};

SelectionSort() ; 

// rated
// div1: 1600 <= r <= 2999
// div2: 1200 <= r <= 2399

// test1
// 2000 1
//
// Yes

package main

import (
	"fmt"
)

func main() {
	var rate, div int
	fmt.Scanf("%d %d", &rate, &div)

	if (div == 1 && rate >= 1600 && rate <= 2999) || (div == 2 && rate >= 1200 && rate <= 2399) {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}
}

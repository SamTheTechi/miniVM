#include <stdio.h>

int main(){
  int i;
  scanf("%d", &i);
  if (i % 2 == 0){
    for (int k = 0; k < 10; k++){
      printf("%d",k);
    }
  } else {
    for (int k = 10; k >= 0; k--){
      printf("%d",k);
    }
  }
}

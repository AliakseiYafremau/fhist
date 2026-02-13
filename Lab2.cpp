#include <iostream>
int main() {
    //Initial menu for user
    std::cout << "1. Print character given int \n";
    std::cout << "2. Print integer value given char \n";
    std::cout << "3. Print ASCII table \n";
    std::cout << "4. Exit program \n";
    int answer;
    std::cin >> answer;

    switch(answer){
        case 1:{
            std::cout << "Enter the integer:";
            int num;
            std::cin >> num;
            if (num<33||num>126){
                std::cout << "Non printable character";
            }else{
                char num2 = char(num);
                std::cout << "The character is: " << num2 << "\n";}}
        case 2:{
            std::cout << "Enter the character:";
            char char1;
            std::cin >> char1;
            int num1 = int(char1);
            std::cout << "The integer number is: "<< num1<< "\n";}
        case 3:{
            std::cout << "-----ASCII TABLE-----\n";
            std::cout << "INT     HEX     CHAR \n";
            for (int i=0; i<33; ++i){
                std::cout << i <<"   "<< std::hex << std::showbase << i <<"   "<< std::dec << char(i) << "Non printable character \n";
            }
            for (int x=33; x<127; ++x){
                std::cout << x <<"   "<< std::hex << std::showbase << x <<"   "<< std::dec << "    " <<char(x) << "\n";

        }




    }

}
}

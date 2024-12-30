public class Counter {

    // Static method to add two integers
    public static int add(int a, int b) {
        return a + b;
    }

    // Main method to test the add method
    public static void main(String[] args) {
        int num1 = 5;
        int num2 = 10;
        int sum = add(num1, num2);
        System.out.println("The sum of " + num1 + " and " + num2 + " is: " + sum);
    }
}
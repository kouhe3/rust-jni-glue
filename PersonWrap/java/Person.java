public class Person {
    private String name;
    private int age;
    //构造函数可以设定age和name
    public Person(String name, int age){
        this.name = name;
        this.age = age;
    }
    //有一个函数介绍自己
    public void introduce(){
        System.out.println("我叫" + name + "，今年" + age + "岁");
    }
}
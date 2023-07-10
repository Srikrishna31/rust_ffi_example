#include <QtWidgets/QPushButton>
#include <QtWidgets/QApplication>
#include <iostream>

int main(int argc, char** argv) {
    std::cout << "Hello World";
    QApplication app(argc, argv);

    QPushButton button("Hello World");
    button.show();

    app.exec();
}

#include <QtWidgets/QPushButton>
#include <QtWidgets/QApplication>

int main(int argc, char** argv) {
    QApplication app(argc, argv);

    QPushButton button("Hello World");
    button.show();

    app.exec();
}

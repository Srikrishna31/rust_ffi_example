#include "main_window.hpp"

extern "C" {
    void hello_world();
}

void MainWindow::onClick() {
    // Call the `hello_world` function to print a message to stdout
    hello_world();
}

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    button = new QPushButton("Click Me", this);

    connect(button, SIGNAL(released()), this, SLOT(onClick()));
}

#include "main_window.hpp"
#include "wrappers.h"
#include <iostream>
#include <QMessageBox>

void MainWindow::onClick() {
    try {
        std::cout << "Creating the request" << std::endl;
//        Request req("https://www.rust-lang.org/");
        auto req = Request("this is an invalid URL");
        std::cout << "Sending Request" << std::endl;
        auto res = req.send();
        std::cout << "Received Response" << std::endl;

        auto raw_body = res.read_body();
        std::cout << std::string(raw_body.begin(), raw_body.end()) << std::endl;
    } catch (const WrapperException & e) {
        QMessageBox::warning(this, "Error", e.what());
    }
}

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
{
    button = new QPushButton("Click Me", this);

    connect(button, SIGNAL(released()), this, SLOT(onClick()));
}

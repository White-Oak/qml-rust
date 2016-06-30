import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

ApplicationWindow
{
    x: 400
    y: 200
    width: 400
    height: 300
    title: "SimpleData"
    Component.onCompleted: visible = true

    ListView {
      anchors.fill: true

        model: listModel
        delegate: Text {
            text: name + ": " + number
        }
    }
}

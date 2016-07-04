import QtQuick 2.2
import QtQuick.Controls 1.2
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

ApplicationWindow {
  width: 400
  height: 300
  title: "of Signals & Slots"
  Component.onCompleted: visible = true

  ColumnLayout {
    anchors.fill: parent
    Text {
      anchors.horizontalCenter: parent.horizontalCenter
      text: "Not set yet"
    }
    Button {
      id: butt
      anchors.horizontalCenter: parent.horizontalCenter
      text: "Notify Rust"
      Component.onCompleted: {
        clicked.connect(test.click)
        test.updateText.connect(updateText)
      }
      function updateText(s) {
        text = s
      }
    }
  }
}

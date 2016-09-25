import QtQuick 2.5;
import QtQuick.Window 2.1;
import QtQuick.Controls 1.4;
import QtQuick.Layouts 1.2;

ApplicationWindow {
    width: 300;
    height: 100;
    Component.onCompleted: visible = true;
    ColumnLayout {
        anchors.fill: parent;
        TextField {
            id: urlField;
            Layout.fillWidth: true
            text: "Anything"
        }
        RowLayout {
            Layout.fillWidth: true
            Item { Layout.fillWidth: true }
            Button {
                text: "Download Page";
                onClicked: logic.downloadPage(urlField.text)
            }
            Item { Layout.fillWidth: true }
        }

        // Three ways of using signals' attributes on connection
        Connections {
            target: logic;
            onPageDownloaded: console.log("Page downloaded " + arguments[0]);
        }

        Connections {
            target: logic;
            onPageDownloaded: console.log("Page downloaded " + arg0);
        }

        Component.onCompleted: {
          logic.pageDownloaded.connect(jsPageDownloaded);
        }

        function jsPageDownloaded(response) {
          console.log("Page downloaded " + response);
        }
    }
}

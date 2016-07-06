import QtQuick 2.0
import QtCharts 2.0
import QtQuick.Layouts 1.0
import QtQuick.Controls 1.3
ApplicationWindow {
  visible: true
  title: "Architect View"
  minimumWidth: 1200
  minimumHeight: 800 + 100
  x: 400
  y: 100
  ChartView {
    title: "Language stats"
    anchors.fill: parent
    antialiasing: true

    PercentBarSeries {
      id: barS
      axisX: BarCategoryAxis { categories: ["2005", "2006"] }
      BarSet { label: "Bob"; values: [2, 2] }
      BarSet { label: "Susan"; values: [5, 1] }
      BarSet { label: "James"; values: [3, 5] }
      Component.onCompleted: {
        console.log(values)
        values.forEach(function(item, index) {
          barS.append(item[0], item[1]);
        });
      }
    }
  }
}

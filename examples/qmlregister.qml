import QtQuick 2.2
import TestModule 1.0

Item {
  TestRsObject{
    id: rsRegistered

    name: "Oak"
  }
  Component.onCompleted: {
    console.log(rsRegistered.name)
    rsRegistered.name = "This is working"
    console.log(rsRegistered.name)
    rsRegistered.assure_everything_okay()
    console.log(TestRsSingleton.temp)
  }
}

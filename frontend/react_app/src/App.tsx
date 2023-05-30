import createEngine, {
    DiagramModel,
} from "@projectstorm/react-diagrams";
import { CanvasWidget } from "@projectstorm/react-canvas-core";
import "./App.css";

import { SvgNodeFactory, SvgNodeModel } from "./nodes/SvgNode";
import { createItem } from "./items/ElectricItemFactory";

function addItem(type: string, model: DiagramModel) {
    const item = createItem(type);
    if (!item) {
        return;
    }
    var node = new SvgNodeModel({
        item: item,
    });
    node.setPosition(400, 100);
    model.addAll(node);
}


function App() {
    //1) setup the diagram engine
    var engine = createEngine();
    engine.getNodeFactories().registerFactory(new SvgNodeFactory());


    var model = new DiagramModel();

    addItem("resistor", model);
    addItem("capacitor", model);
    addItem("inductor", model);

    //5) load model into engine
    engine.setModel(model);
    return <CanvasWidget engine={engine} className="canvas" />;
}

export default App;
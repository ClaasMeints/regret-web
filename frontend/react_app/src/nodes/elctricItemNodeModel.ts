import { DefaultPortModel, NodeModel } from "@projectstorm/react-diagrams";


class ElectricItemNodeModel extends NodeModel {
    constructor() {
        super({
            type: "electricItem",
        });
        this.addPort(new DefaultPortModel(false, "out-1", "out-1"));
    }
}

export default ElectricItemNodeModel;
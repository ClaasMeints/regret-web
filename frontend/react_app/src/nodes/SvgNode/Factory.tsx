import { AbstractReactFactory, DiagramEngine } from "@projectstorm/react-diagrams";
import { SvgNodeModel } from "./Model";
import { SvgNodeWidget } from "./Widget";

export class SvgNodeFactory extends AbstractReactFactory<SvgNodeModel, DiagramEngine> {
    constructor() {
        super("electricItem");
    }

    generateModel(event: any) {
        return new SvgNodeModel(event);
    }

    generateReactWidget(event: any) {
        return SvgNodeWidget({ node: this.generateModel(event.model), engine: this.engine, size: 200 });
    }
}
import { DefaultPortModel, NodeModel, NodeModelGenerics, PortModelAlignment} from "@projectstorm/react-diagrams";

type Item = {
    name: string,
    svg: string,
}

export class SvgNodeModel extends NodeModel {
    private _item: Item;
    constructor(options: {
        item: Item
    }) {
        super(Object.assign({
            type: "electricItem",
            name: options.item.name,
        }, options));
        this._item = options.item;
        super.addPort(new DefaultPortModel({
            in: true,
            name: "in-1",
            label: "in-1",
            alignment: PortModelAlignment.LEFT,
        }));
        super.addPort(new DefaultPortModel({
            in: false,
            name: "out-1",
            label: "out-1",
            alignment: PortModelAlignment.RIGHT,
        }));
    }

    public get item(): Item {
        return this._item;
    }
}
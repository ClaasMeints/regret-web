import { NodeWidget } from "@projectstorm/react-diagrams";
import { DefaultNodeModel } from "@projectstorm/react-diagrams";
import * as React from "react";
import ElectricItemNodeModel from "../nodes/elctricItemNodeModel";

import resistor from "../assets/resistor.svg";

const svgItems = [
    {
        name: "Light Bulb",
        svg: resistor,
    }
];

function ElectricItemNodeWidget(props: { node: ElectricItemNodeModel }) {
    const [selectedItem, setSelectedItem] = React.useState(svgItems[0]); // Initialize with the first item

    const handleItemChange = (event: React.ChangeEvent<HTMLSelectElement>) => {
        const selectedItem = svgItems.find((item) => item.name === event.target.value);
        if (selectedItem) {
            setSelectedItem(selectedItem);
        }
    };

    return (
        <div className="custom-node">
            <select value={selectedItem.name} onChange={handleItemChange}>
                {svgItems.map((item) => (
                    <option key={item.name} value={item.name}>
                        {item.name}
                    </option>
                ))}
            </select>
            <div className="svg-container" dangerouslySetInnerHTML={{ __html: selectedItem.svg }} />
            <div className="name">{node.getOptions().name}</div>
        </div>
    );
};
import React, { useState } from "react";

const Dropdown = ({ text, options, defaultValue }) => {
    const [selected, setSelected] = useState(defaultValue);
    const handleSelect = (e) => {
        console.log(e.target.value);
        setSelected(e.target.value);
    }
    return (
        <div className="dropdown">
            <span className="dropdown-text">{text}</span>
            <select className="dropdown-select" name="select" onChange={handleSelect}>
                {options.map(n => { 
                    return (<option value={n} selected={selected === n}>{n}</option>);
                })}
            </select>
        </div>
    );
}

export default Dropdown;

import React from "react";
import './screen.css';

const Screen = ({ sizex, sizey }) => {
    const style = {
        'width': `${sizex}px`,
        'heigh': `${sizey}px`,
    };
    return(
        <div className="screen" style={style}>
        </div>
    );
}

export default Screen;

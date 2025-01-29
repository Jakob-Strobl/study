import { useEffect, useRef } from "react";
import { createPortal } from "react-dom";

const Modal = ({ children }) => {
  // Refs are Frozen objects
  // Only thing you can modify is current
  // We are only going to create one div, and use the same div every render.
  const elRef = useRef(null);
  if (!elRef.current) {
    elRef.current = document.createElement("div");
  }

  useEffect(() => {
    const modalRoot = document.getElementById("modal");
    modalRoot.appendChild(elRef.current);
    // If we left this as is it has a memory leak
    // because the elements will be appended but theres no onDismount cleanup call for the effect
    // return;

    return () => modalRoot.removeChild(elRef.current);
  }, []);

  // Allows you render any children in Modal into the portal automatically
  return createPortal(<div>{children}</div>, elRef.current);
};

export default Modal;

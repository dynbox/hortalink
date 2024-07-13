import React from "react";
import Geolocation from "../stores/Geolocation";

class NotAvailableError extends Error {
    constructor(msg: string) {
        super(msg);

        Object.setPrototypeOf(this, NotAvailableError.prototype);
    }

}

function watchPosition() {
    if("geolocation" in navigator) {
        const WatchId = navigator.geolocation.watchPosition((position) => {
            Geolocation.position.set([position.coords.latitude, position.coords.longitude])
            Geolocation.state.set("updated")

        }, (error) => {
            if(error.code === error.PERMISSION_DENIED) {
                Geolocation.state.set("denied")
            }
        }, {
            enableHighAccuracy: true
        })

        return WatchId
    } else {
        throw new NotAvailableError("Geolocation não é suportado pelo navegador.")   
    }
}

export default {
    watchPosition
}
import Geolocation from "../stores/Geolocation";

export default function degreesToKm(distanceInDegrees: number) {
    const kmPerDegree = 111.32; // km por grau de latitude

    const userLatitude = Geolocation.position.get()[0]

    const distanceInMeters = distanceInDegrees * kmPerDegree * Math.cos(userLatitude * Math.PI / 180);
    return (distanceInMeters / 1000).toFixed(2);
}
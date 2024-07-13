export default function degreesToKm(distanceInDegrees: number, userLatitude: number) {
    const kmPerDegree = 111.32; // km por grau de latitude
    const distanceInMeters = distanceInDegrees * kmPerDegree * Math.cos(userLatitude * Math.PI / 180);
    return (distanceInMeters / 1000).toFixed(2);
}
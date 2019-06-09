class SeriesResponse {
    public id: number | null;
    public capitulos: string | null;
    public categoria: string | null;
    public fansub: string | null;
    public idioma: string | null;
    public name: string | null;

    constructor() {
        this.id = null;
        this.capitulos = null;
        this.categoria = null;
        this.fansub = null;
        this.idioma = null;
        this.name = null;
    }
}

export default SeriesResponse;

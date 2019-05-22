class SeriesResponse {
    id: number | null;
    capitulos: string | null;
    categoria: string | null;
    fansub: string | null;
    idioma: string | null;
    name: string | null;

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
